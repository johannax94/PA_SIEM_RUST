import requests
import random
import time
import uuid

URL = "http://127.0.0.1:3000/logs"

source_types = ["proxy", "system", "firewall"]

def generate_log():

    source_type = random.choice(source_types)

    if source_type == "proxy":
        return {
            "source_type": "proxy",
            "source_name": "proxy01",
            "level": "info",
            "event_type": "http_request",
            "message": "HTTP request",
            "metadata": {
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
            "source_type": "system",
            "source_name": "server01",
            "level": "warn",
            "event_type": random.choice([
                "login_failed",
                "login_success"
            ]),
            "message": "Auth event",
            "metadata": {
                "user": random.choice(["admin", "root", "user"]),
                "src_ip": f"10.0.0.{random.randint(1,50)}"
            }
        }

    else:
        return {
            "source_type": "firewall",
            "source_name": "fw01",
            "level": "warn",
            "event_type": "blocked_connection",
            "message": "Connection blocked",
            "metadata": {
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
        except:
            pass

        if i % 100 == 0:
            print(f"{i} logs sent")

if __name__ == "__main__":
    send_logs(10000)

    import threading

def worker():
    send_logs(2000)

threads = []

for _ in range(5):
    t = threading.Thread(target=worker)
    t.start()
    threads.append(t)

for t in threads:
    t.join()