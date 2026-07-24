import requests
import random
import time
import sys

URL = "http://127.0.0.1:3000/logs"

# IMPORTANT : le backend (IncomingLog dans src/models/log.rs) attend EXACTEMENT
# ces champs -> source_name, vendor, hostname, username, ip_address, event_type,
# severity, message, raw_log. Les règles lisent username / ip_address / hostname
# au niveau RACINE (pas dans raw_log) : on les envoie donc en top-level.

SESSION = requests.Session()  # réutilise la connexion TCP -> beaucoup plus rapide

# --------------------------------------------------------------------------
# Pools d'entités COHÉRENTES : les mêmes machines/users/IP reviennent, comme
# dans un vrai réseau d'entreprise (c'est ça qui fait "réaliste", pas le volume).
# --------------------------------------------------------------------------
HOSTNAMES = [
    "PC-COMPTA-01", "PC-COMPTA-02", "PC-RH-01", "PC-DIR-01", "PC-DEV-03",
    "PC-ACCUEIL-01", "SRV-AD-01", "SRV-FILE-01", "SRV-WEB-01", "SRV-BACKUP-01",
]
USERS = [
    "j.dupont", "m.martin", "p.lefevre", "s.moreau", "a.bernard", "c.petit",
    "admin", "administrateur", "svc-backup", "svc-sql",
]
INT_IPS = [f"10.0.0.{i}" for i in range(10, 60)] + [f"192.168.1.{i}" for i in range(10, 60)]
EXT_IPS = ["8.8.8.8", "1.1.1.1", "140.82.112.3", "52.97.140.10", "142.250.75.196"]

SOURCES = [
    ("AD-DC01", "Windows"),
    ("fw-perimeter", "Fortinet"),
    ("proxy-web", "Squid"),
    ("edr-agent", "CrowdStrike"),
    ("srv-file01", "Windows"),
]

BENIGN_CMDS = [
    "C:\\Windows\\explorer.exe",
    "outlook.exe /recycle",
    "chrome.exe --profile-directory=Default",
    "teams.exe --process-start-args",
    "svchost.exe -k netsvcs",
    "powershell.exe -File C:\\scripts\\backup_quotidien.ps1",  # 1 flag = bénin
]


def post(log):
    try:
        SESSION.post(URL, json=log, timeout=3)
    except Exception:
        pass


# --------------------------------------------------------------------------
# Génération d'un log BÉNIN (le bruit de fond, ~95 % du trafic).
# Distribution pondérée : surtout de l'auth OK et du web, un peu de tout le reste.
# --------------------------------------------------------------------------
def benign_log():
    kind = random.choices(
        ["auth_ok", "web", "proc", "dns", "fw_ok", "auth_fail"],
        weights=[34, 26, 18, 8, 8, 6],
        k=1,
    )[0]

    host = random.choice(HOSTNAMES)
    user = random.choice(USERS)

    if kind == "auth_ok":
        return {
            "source_name": "AD-DC01", "vendor": "Windows", "hostname": host,
            "username": user, "ip_address": random.choice(INT_IPS),
            "event_type": "login_success", "severity": "info",
            "message": "Ouverture de session réussie (4624)",
            "raw_log": {"logon_type": random.choice(["2", "3", "7"]), "country": "FR"},
        }

    if kind == "web":
        return {
            "source_name": "proxy-web", "vendor": "Squid", "hostname": host,
            "username": user, "ip_address": random.choice(INT_IPS),
            "event_type": "network_flow", "severity": "info",
            "message": "Requête HTTP autorisée",
            "raw_log": {
                "dest_ip": random.choice(INT_IPS),  # trafic interne -> pas d'exfil
                "bytes_out": random.randint(1_000, 500_000),
                "url": random.choice(["intranet.local", "sharepoint.local", "erp.local"]),
                "status_code": random.choice([200, 200, 200, 304]),
            },
        }

    if kind == "proc":
        return {
            "source_name": "edr-agent", "vendor": "CrowdStrike", "hostname": host,
            "username": user, "ip_address": random.choice(INT_IPS),
            "event_type": "process_create", "severity": "info",
            "message": random.choice(BENIGN_CMDS),
            "raw_log": {"parent_process": "explorer.exe"},
        }

    if kind == "dns":
        return {
            "source_name": "AD-DC01", "vendor": "Windows", "hostname": host,
            "username": user, "ip_address": random.choice(INT_IPS),
            "event_type": "dns_query", "severity": "info",
            "message": "Requête DNS",
            "raw_log": {"query": random.choice(["update.microsoft.com", "erp.local", "time.windows.com"])},
        }

    if kind == "fw_ok":
        return {
            "source_name": "fw-perimeter", "vendor": "Fortinet", "hostname": "fw01",
            "username": None, "ip_address": random.choice(INT_IPS),
            "event_type": "allowed_connection", "severity": "info",
            "message": "Connexion autorisée",
            "raw_log": {"dest_ip": random.choice(EXT_IPS), "dest_port": random.choice([443, 80, 53])},
        }

    # auth_fail bénin isolé (un employé qui se trompe) : réparti sur users/IP
    # variés -> ne concentre pas assez pour déclencher failed_login/password_spray.
    return {
        "source_name": "AD-DC01", "vendor": "Windows", "hostname": host,
        "username": user, "ip_address": random.choice(INT_IPS),
        "event_type": "login_failed", "severity": "warning",
        "message": "Échec d'ouverture de session (4625)",
        "raw_log": {"logon_type": "2", "status": "0xC000006A"},
    }


# --------------------------------------------------------------------------
# Rafales d'ATTAQUE (intercalées) : concentrées pour déclencher une règle,
# afin que la page Alertes se rafraîchisse aussi en direct.
# --------------------------------------------------------------------------
def attack_rdp_bruteforce():
    ip = "45.137.21.9"  # IP attaquant externe
    for _ in range(22):
        post({
            "source_name": "SRV-AD-01", "vendor": "Windows", "hostname": "SRV-AD-01",
            "username": "administrateur", "ip_address": ip,
            "event_type": "login_failed", "severity": "warning",
            "message": "Échec logon RDP (4625)",
            "raw_log": {"logon_type": "10", "status": "0xC000006A"},
        })


def attack_port_scan():
    ip = "45.137.21.50"
    for p in range(3389, 3389 + 26):  # 26 ports distincts
        post({
            "source_name": "fw-perimeter", "vendor": "Fortinet", "hostname": "fw01",
            "username": None, "ip_address": ip,
            "event_type": "blocked_connection", "severity": "warning",
            "message": "Connexion bloquée",
            "raw_log": {"dest_ip": "192.168.1.10", "dest_port": p},
        })


def attack_powershell():
    post({
        "source_name": "edr-agent", "vendor": "CrowdStrike", "hostname": "PC-DIR-01",
        "username": "j.dupont", "ip_address": random.choice(INT_IPS),
        "event_type": "process_create", "severity": "high",
        "message": "powershell.exe -nop -w hidden -enc SQBFAFgAKAAuAC4A",
        "raw_log": {"parent_process": "winword.exe"},
    })


ATTACKS = [attack_rdp_bruteforce, attack_port_scan, attack_powershell]


# --------------------------------------------------------------------------
# Modes d'émission
# --------------------------------------------------------------------------
def stream_duration(duration_s: float, rate_per_sec: float, with_attacks: bool = True):
    """Flux réaliste pendant une durée fixe (idéal démo auto-refresh)."""
    delay = 1.0 / rate_per_sec
    end = time.time() + duration_s
    next_attack = time.time() + 25
    sent = 0
    print(f"Flux réaliste : {rate_per_sec:g} logs/s pendant {duration_s:g}s "
          f"({'avec' if with_attacks else 'sans'} rafales d'attaque)")
    while time.time() < end:
        post(benign_log())
        sent += 1
        if sent % 25 == 0:
            print(f"  {sent} logs bénins envoyés…")
        if with_attacks and time.time() >= next_attack:
            fn = random.choice(ATTACKS)
            print(f"  >> rafale d'attaque : {fn.__name__}")
            fn()
            next_attack = time.time() + random.randint(35, 55)
        time.sleep(delay)
    print(f"\nTerminé. {sent} logs bénins (+ rafales) envoyés en {duration_s:g}s.")


def stream_logs(rate_per_sec: float):
    """Flux continu illimité (Ctrl+C pour arrêter)."""
    delay = 1.0 / rate_per_sec
    sent = 0
    print(f"Flux continu : ~{rate_per_sec:g} logs/s (Ctrl+C pour arrêter)")
    try:
        while True:
            post(benign_log())
            sent += 1
            if sent % 25 == 0:
                print(f"{sent} logs envoyés…")
            time.sleep(delay)
    except KeyboardInterrupt:
        print(f"\nArrêté. {sent} logs envoyés au total.")


def send_count(n: int):
    """Lot unique de N logs, le plus vite possible."""
    for i in range(n):
        post(benign_log())
        if i % 100 == 0:
            print(f"{i} logs envoyés…")
    print(f"Terminé. {n} logs envoyés.")


def _flag(args, name, default):
    if name in args:
        i = args.index(name)
        if i + 1 < len(args):
            return args[i + 1]
    return default


if __name__ == "__main__":
    # Usage :
    #   python log_generator.py --duration 180            -> démo 3 min (8 logs/s + attaques)
    #   python log_generator.py --duration 120 --rate 15  -> 2 min à 15 logs/s
    #   python log_generator.py --duration 180 --no-attacks
    #   python log_generator.py --continuous 10           -> flux continu 10 logs/s
    #   python log_generator.py --count 20000             -> gros lot ponctuel
    args = sys.argv[1:]

    if "--duration" in args:
        dur = float(_flag(args, "--duration", 180))
        rate = float(_flag(args, "--rate", 8))
        stream_duration(dur, rate, with_attacks="--no-attacks" not in args)
    elif "--continuous" in args:
        rate = float(_flag(args, "--continuous", 2))
        stream_logs(rate)
    elif "--count" in args:
        send_count(int(_flag(args, "--count", 1000)))
    else:
        send_count(600)
