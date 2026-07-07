"""
Suite de tests d'intégration des règles de détection du SIEM.

Pour chaque règle : on injecte via POST /logs le(s) log(s) qui doivent la
déclencher, puis on vérifie via GET /alerts que l'alerte attendue apparaît.

Prérequis :
  - backend + base lancés, avec le schéma migré (colonnes vendor, search_vector…)
  - un compte admin/admin (créé via /auth/register)
  - `pip install requests`

Usage :
  python test_rules.py
"""

import subprocess
import sys
import time

import requests

API = "http://localhost:3000"
ADMIN = {"username": "admin", "password": "admin"}

# Modèle d'un log valide (doit matcher IncomingLog côté backend)
BASE_LOG = {
    "source_name": "test",
    "vendor": None,
    "hostname": None,
    "username": None,
    "ip_address": None,
    "event_type": "generic",
    "severity": "info",
    "message": "",
    "raw_log": {},
}


def login() -> str:
    r = requests.post(f"{API}/auth/login", json=ADMIN, timeout=5)
    r.raise_for_status()
    return r.json()["token"]


def clear_alerts() -> None:
    """Vide la table alerts pour repartir propre (évite la déduplication)."""
    try:
        subprocess.run(
            ["docker", "exec", "medtech-db", "psql", "-U", "postgres",
             "-d", "medtech_siem", "-c", "TRUNCATE alerts;"],
            check=True, capture_output=True,
        )
        print("Table alerts vidée.\n")
    except Exception as e:  # noqa: BLE001
        print(f"(!) Impossible de vider alerts automatiquement : {e}\n")


def send_log(**fields) -> None:
    log = dict(BASE_LOG)
    log.update(fields)
    requests.post(f"{API}/logs", json=log, timeout=5)


def send_many(n: int, **fields) -> None:
    for _ in range(n):
        send_log(**fields)


def alert_rule_names(token: str) -> set:
    r = requests.get(
        f"{API}/alerts",
        headers={"Authorization": f"Bearer {token}"},
        timeout=5,
    )
    return {a.get("rule_name") for a in r.json()}


# ---- Définition des scénarios : (libellé, règle attendue, fonction d'injection) ----
def scenarios():
    return [
        # 12 échecs sur le compte "administrator" (privilégié) -> critical.
        ("Brute-force compte privilégié (12 échecs)", "multiple_failed_logins",
         lambda: send_many(12, source_name="srv-ad", event_type="login_failed",
                           severity="warning", username="administrator", ip_address="10.0.0.5",
                           message="4625 bad password")),

        ("Password spraying (5 users, même IP)", "password_spray",
         lambda: [send_log(source_name="srv-ps", event_type="login_failed",
                           severity="warning", ip_address="10.0.0.6",
                           username=f"user{i}", message="bad password")
                  for i in range(5)]),

        ("Account compromise (échecs puis succès)", "account_compromise",
         lambda: (send_many(5, source_name="srv-ac", event_type="login_failed",
                            severity="warning", username="bob", message="fail"),
                  send_log(source_name="srv-ac", event_type="login_success",
                           severity="info", username="bob", message="ok"))),

        ("PowerShell suspect (-enc)", "powershell_suspect",
         lambda: send_log(source_name="pc-ps", event_type="process_create",
                          severity="high", message="powershell.exe -nop -enc SQBFAFgA")),

        ("Exécution cmd.exe", "cmd_execution",
         lambda: send_log(source_name="pc-cmd", event_type="process_create",
                          severity="medium", message="cmd.exe /c whoami")),

        ("Scan réseau (50 connexions bloquées)", "network_scan",
         lambda: send_many(50, source_name="fw-scan", event_type="blocked_connection",
                           severity="warning", ip_address="10.0.0.99", message="blocked")),

        # 22 fichiers renommés avec l'extension ransomware ".locked".
        ("Ransomware (22 fichiers .locked)", "ransomware",
         lambda: send_many(22, source_name="fs-partage", event_type="file_renamed",
                           severity="warning", message="file renamed",
                           raw_log={"filename": "document.locked", "new_extension": "locked"})),

        ("RDP depuis pays inhabituel", "rdp_foreign_country",
         lambda: send_log(source_name="srv-rdp", event_type="rdp_login",
                          severity="high", username="admin",
                          message="rdp", raw_log={"country": "RU"})),

        # 6 flux de 100 Mo vers une IP externe (8.8.8.8) = 600 Mo cumulés > 500 Mo.
        ("Exfiltration (>500 Mo cumulés vers l'externe)", "data_exfiltration",
         lambda: send_many(6, source_name="wks-exfil", event_type="network_flow",
                           severity="info", message="outbound flow",
                           raw_log={"bytes_out": 100_000_000, "dest_ip": "8.8.8.8"})),

        ("Impossible travel (2 pays)", "impossible_travel",
         lambda: (send_log(source_name="srv-it", event_type="login_success",
                           severity="info", username="alice", message="ok",
                           raw_log={"country": "FR"}),
                  send_log(source_name="srv-it", event_type="login_success",
                           severity="info", username="alice", message="ok",
                           raw_log={"country": "RU"}))),

        ("DNS tunneling (100 requêtes DNS)", "dns_tunneling",
         lambda: send_many(100, source_name="srv-dns", event_type="dns_query",
                           severity="info", message="dns query")),

        ("Beaconing C2 (30 connexions sortantes)", "beaconing_c2",
         lambda: send_many(30, source_name="srv-c2", event_type="outbound_connection",
                           severity="info", ip_address="10.0.0.42", message="beacon")),

        ("Privilege escalation", "privilege_escalation",
         lambda: send_log(source_name="srv-pe", event_type="privilege_escalation",
                          severity="high", message="SeDebugPrivilege enabled")),

        ("Pass-the-Hash (logon type 9 NTLM)", "pass_the_hash",
         lambda: send_log(source_name="srv-pth", event_type="logon", severity="high",
                          message="NTLM logon", raw_log={"logon_type": "9"})),


        # Règle approfondie : 22 échecs RDP (4625, logon_type 10) depuis une IP
        # EXTERNE (203.0.113.45 = plage de doc, publique) sur le même serveur.
        ("Brute-force RDP externe (MITRE T1110/T1021)", "rdp_bruteforce_external",
         lambda: send_many(22, source_name="srv-rdp01", event_type="login_failed",
                           severity="warning", username="administrator",
                           ip_address="203.0.113.45", message="4625 RDP logon failed",
                           raw_log={"event_id": 4625, "logon_type": "10",
                                    "status": "0xC000006A"})),
    ]


def main() -> int:
    print("== Test des règles de détection ==\n")
    clear_alerts()

    token = login()
    tests = scenarios()

    # On envoie tous les scénarios…
    for label, _, inject in tests:
        print(f"-> injection : {label}")
        inject()

    # …puis on attend que les alertes se STABILISENT (l'ingestion est asynchrone
    # et le volume de logs peut mettre plusieurs secondes à être traité).
    print("\nAttente du traitement (ingestion + règles)…")
    found: set = set()
    stable = 0
    for _ in range(30):  # jusqu'à ~30 s
        time.sleep(1)
        current = alert_rule_names(token)
        if current == found:
            stable += 1
            if stable >= 3:  # 3 s sans nouvelle alerte -> terminé
                break
        else:
            stable = 0
            found = current

    print("\n== Résultats ==")
    ok = 0
    for label, expected, _ in tests:
        hit = expected in found
        ok += hit
        print(f"[{'OK ' if hit else 'FAIL'}] {label}  ->  {expected}")

    total = len(tests)
    print(f"\n{ok}/{total} règles déclenchées.")
    return 0 if ok == total else 1


if __name__ == "__main__":
    sys.exit(main())
