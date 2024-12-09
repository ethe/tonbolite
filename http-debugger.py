from http import server


class MyHTTPRequestHandler(server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.allow_opfs()

        server.SimpleHTTPRequestHandler.end_headers(self)

    def allow_opfs(self):
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")


if __name__ == "__main__":
    server.test(HandlerClass=MyHTTPRequestHandler)
