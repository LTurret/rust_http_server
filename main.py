import socket

from enum import Enum


class HttpMethod(Enum):
    GET = "GET"
    POST = "POST"


class HttpRequest:
    def __init__(
        self, method: HttpMethod, url: str, headers: dict[str, str] = None, body=None
    ) -> None:
        self.method: str = method.name
        self.url: str = url
        self.headers: dict[str, str] = headers or {}
        self.body = body

    def build_request(self) -> bytes:
        request_line: str = f"{self.method} {self.url} HTTP/1.1\n"
        headers: str = "".join(
            f"{key}: {value}\n" for key, value in self.headers.items()
        )
        request_content: str = f"{request_line}{headers}\n{self.body or ''}"
        packet: bytes = request_content.encode("utf-8")
        return packet


class HttpClient:
    def __init__(self, host: str, port: int = 80) -> None:
        self.host: str = host
        self.port: int = port

    def send_request(self, http_reqest: HttpRequest) -> str:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as stream:
            stream.connect((self.host, self.port))
            stream.sendall(http_reqest.build_request())
            response: bytes = stream.recv(4096)
        return response.decode()


if __name__ == "__main__":
    headers: dict[str, str] = {"Host": "example.com", "User-Agent": "RustHttpClient"}
    request: HttpRequest = HttpRequest(HttpMethod.GET, "/", headers)
    client: HttpClient = HttpClient("example.com")
    response: str = client.send_request(request)
    print(response)