import http.server
import json
import pathlib
import re
import socket


class TestRuntimeFailureHandler(http.server.BaseHTTPRequestHandler):
    def log_message(self, format, *args):
        return None

    def do_GET(self):
        self.respond()

    def do_POST(self):
        size = int(self.headers.get("Content-Length", "0"))
        if not 0 < size <= 65536:
            self.send_error(400)
            return
        request = json.loads(self.rfile.read(size))
        if not isinstance(request.get("message"), str):
            self.send_error(400)
            return
        self.respond()

    def respond(self):
        selected, failure, service, route = self.path.lstrip("/").split("/", 3)
        path = "/" + route
        if self.command == "POST" and path == self.server.route_paths["notification"]:
            operation = 4
            body = {"id": "00000000-0000-4000-8000-000000000001"}
            status = 201
        elif self.command == "GET" and path in self.server.route_paths.values():
            operation = (0 if service == "application" else 2) + int(
                path == self.server.route_paths["ready"]
            )
            body = {"status": "ok", "components": []}
            status = 200
        else:
            self.send_error(404)
            return
        if operation == int(selected):
            if failure == "request":
                self.connection.shutdown(socket.SHUT_RDWR)
                self.connection.close()
                return
            if failure == "status":
                status = 503
            if failure == "unhealthy":
                body["status"] = "degraded"
            payload = b"{" if failure == "response" else json.dumps(body).encode()
        else:
            payload = json.dumps(body).encode()
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(payload)))
        self.end_headers()
        self.wfile.write(payload)


if __name__ == "__main__":
    root = pathlib.Path(__file__).resolve().parents[2]
    routes = {
        "live": "common_routes/src/health_live_route.rs",
        "ready": "common_routes/src/health_ready_route.rs",
        "notification": "notification_service_contract/src/create_notification_route.rs",
    }
    def read_route(item):
        name, source = item
        matches = re.findall(r'\bpath\s*=\s*"([^"]+)"', (root / source).read_text())
        if len(matches) != 1:
            raise ValueError(source)
        return name, matches[0]

    route_paths = dict(map(read_route, routes.items()))
    with http.server.HTTPServer(("127.0.0.1", 0), TestRuntimeFailureHandler) as server:
        server.route_paths = route_paths
        print(f"127.0.0.1:{server.server_port}", flush=True)
        server.serve_forever()
