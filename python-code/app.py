import socket


def main():

    host = "127.0.0.1"
    port = 7878

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:

        sock.connect((host, port))
        sock.sendall(b"Hello from python")

        received_data = sock.recv(512)

        print(f"Got the resp which says, {received_data.decode('utf-8')}")

    sock.close


if __name__ == "__main__":
    main()
