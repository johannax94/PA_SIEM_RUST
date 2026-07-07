import requests
import random
import time
import uuid

URL = "http://127.0.0.1:3000/logs"

# IMPORTANT : le backend (IncomingLog dans src/models/log.rs) attend EXACTEMENT
# ces champs -> source_name, event_type, severity, message, raw_log.
# ANCIENNE VERSION (cassée) : elle envoyait "level" (au lieu de "severity")
# et "metadata" (au lieu de "raw_log"), ce qui faisait rejeter chaque log en 422
# -> aucun log n'était ingéré.

source_types = ["proxy", "system", "firewall"]

def generate_log():

    source_type = random.choice(source_types)

    if source_type == "proxy":
        return {
            "source_name": "proxy01",
            "event_type": "http_request",
            "severity": "info",          # anciennement "level"
            "message": "HTTP request",
            "raw_log": {                  # anciennement "metadata"
                "source_type": "proxy",
                "src_ip": f"10.0.0.{random.randint(1,50)}",
                "url": random.choice([
                    "https://google.com",
                    "https://github.com",
                    "https://example.com"
                ]),
                "status_code": random.choice([200, 403, 500])
            }
        }

    elif source_type == "system":
        return {
            "source_name": "server01",
            "event_type": random.choice([
                "login_failed",
                "login_success"
            ]),
            "severity": "warning",        # anciennement "level": "warn"
            "message": "Auth event",
            "raw_log": {                  # anciennement "metadata"
                "source_type": "system",
                "user": random.choice(["admin", "root", "user"]),
                "src_ip": f"10.0.0.{random.randint(1,50)}"
            }
        }

    else:
        return {
            "source_name": "fw01",
            "event_type": "blocked_connection",
            "severity": "warning",        # anciennement "level": "warn"
            "message": "Connection blocked",
            "raw_log": {                  # anciennement "metadata"
                "source_type": "firewall",
                "src_ip": f"10.0.0.{random.randint(1,50)}",
                "dest_ip": "8.8.8.8",
                "port": random.randint(20, 1024)
            }
        }


def send_logs(n):

    for i in range(n):
        log = generate_log()

        try:
            requests.post(URL, json=log)
        except Exception:
            pass

        if i % 50 == 0:
            print(f"{i} logs sent")


def stream_logs(rate_per_sec: float):
    """Mode SIEM temps réel : émet des logs en continu (Ctrl+C pour arrêter)."""
    delay = 1.0 / rate_per_sec
    sent = 0
    print(f"Flux continu : ~{rate_per_sec:g} logs/s (Ctrl+C pour arrêter)")
    try:
        while True:
            try:
                requests.post(URL, json=generate_log())
                sent += 1
                if sent % 25 == 0:
                    print(f"{sent} logs envoyés…")
            except Exception:
                pass
            time.sleep(delay)
    except KeyboardInterrupt:
        print(f"\nArrêté. {sent} logs envoyés au total.")


if __name__ == "__main__":
    # ANCIENNE VERSION : send_logs(10000) en bloquant PUIS 5 threads x 2000
    # (le bloc threading était hors du if __name__ et s'exécutait même à l'import).
    #
    # Usage :
    #   python log_generator.py                  -> lot unique (~600 logs, 5 threads)
    #   python log_generator.py --continuous     -> flux continu à 2 logs/s
    #   python log_generator.py --continuous 10  -> flux continu à 10 logs/s
    import sys
    import threading

    if "--continuous" in sys.argv:
        idx = sys.argv.index("--continuous")
        rate = float(sys.argv[idx + 1]) if len(sys.argv) > idx + 1 else 2.0
        stream_logs(rate)
    else:
        def worker():
            send_logs(120)

        threads = [threading.Thread(target=worker) for _ in range(5)]
        for t in threads:
            t.start()
        for t in threads:
            t.join()

        print("Terminé.")
