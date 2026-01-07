import socket
from threading import Thread

SERVER_HOST = "127.0.0.1"
SERVER_PORT = 7878

UDP_HOST = "127.0.0.1"
UDP_PORT = 7879


def tcp_client():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((SERVER_HOST, SERVER_PORT))
        s.settimeout(5)
        s.sendall(f"STREAM {UDP_HOST}:{UDP_PORT} T1,T9\n".encode("utf-8"))


def udp_client():
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as s:
        s.setblocking(False)
        s.bind((UDP_HOST, UDP_PORT))
        while True:
            try:
                data = s.recv(1004)
                resp = data.decode("utf-8")
                if resp == "BYE":
                    break
                print(f"Ответ: {resp}")
            except Exception:
                pass


t1 = Thread(target=tcp_client)
t1.start()
t2 = Thread(target=udp_client)
t2.start()


t2.join()
t1.join()
print("Соединение закрыто")
