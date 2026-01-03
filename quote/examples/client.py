import socket
from threading import Thread

HOST = "127.0.0.1"
PORT = 7878


def client(s: socket.socket):
    while True:
        try:
            data = s.recv(512)
            resp = data.decode("utf-8")
            if resp == "BYE":
                break
            print(f"Ответ: {resp}")
        except Exception:
            pass


with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    s.settimeout(5)
    t = Thread(target=client, args=(s,))
    t.start()

    while True:
        mes = input()
        if mes.lower() == "exit":
            break
        s.sendall((mes + "\n").encode("utf-8"))

    t.join()

print("Соединение закрыто")
