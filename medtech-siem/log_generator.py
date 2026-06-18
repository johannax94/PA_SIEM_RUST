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


if __name__ == "__main__":
    # ANCIENNE VERSION : send_logs(10000) en bloquant PUIS 5 threads x 2000
    # (le bloc threading était hors du if __name__ et s'exécutait même à l'import).
    # Version raisonnable pour une démo : ~600 logs répartis sur 5 threads.
    import threading

    def worker():
        send_logs(120)

    threads = [threading.Thread(target=worker) for _ in range(5)]
    for t in threads:
        t.start()
    for t in threads:
        t.join()

    print("Terminé.")
