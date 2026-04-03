from http.server import HTTPServer, BaseHTTPRequestHandler

class MockHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        # 1. WAF Simulation (Too Many Requests)
        if self.path.startswith('/waf'):
            self.send_response(429)
            self.send_header('Content-type', 'text/plain')
            self.end_headers()
            self.wfile.write(b"WAF Block: Too Many Requests")
            return

        # 2. Known Paths (Normal Behavior)
        if self.path == '/':
            self.send_response(200)
            self.send_header('Content-type', 'text/html')
            self.end_headers()
            self.wfile.write(b"<html><head><title>Home</title></head><body><h1>Welcome</h1><a href='/admin'>Admin</a><a href='/api/v1'>API</a></body></html>")
        elif self.path == '/admin':
            self.send_response(200)
            self.send_header('Content-type', 'text/html')
            self.end_headers()
            self.wfile.write(b"<html><head><title>Admin Panel</title></head><body><a href='/admin/config.php'>Config</a><a href='/admin/users'>Users</a></body></html>")
        elif self.path == '/admin/config.php':
            self.send_response(200)
            self.send_header('Content-type', 'text/plain')
            self.end_headers()
            self.wfile.write(b"DB_PASSWORD=secret123")
        elif self.path == '/api/v1':
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()
            self.wfile.write(b'{"status":"ok", "version":"1.0"}')
        elif self.path == '/artisan':
            self.send_response(200)
            self.send_header('Content-type', 'text/plain')
            self.end_headers()
            self.wfile.write(b"Laravel Framework 10.0")
        elif self.path == '/manage.py':
            self.send_response(200)
            self.send_header('Content-type', 'text/plain')
            self.end_headers()
            self.wfile.write(b"Django 4.2")
        elif self.path == '/graphql':
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()
            self.wfile.write(b'{"data":{"__schema":{"queryType":{"name":"Query"}}}}')
        elif self.path == '/.env':
            self.send_response(200)
            self.send_header('Content-type', 'text/plain')
            self.end_headers()
            self.wfile.write(b"APP_KEY=base64:xxx\nDB_PASSWORD=root")
        
        # 3. Soft-404 Simulation (Return 200 for any other path with a fixed 404-like body)
        else:
            self.send_response(200)
            self.send_header('Content-type', 'text/html')
            self.end_headers()
            self.wfile.write(b"<html><head><title>404 Not Found</title></head><body>The page you are looking for does not exist but we return 200.</body></html>")

httpd = HTTPServer(('127.0.0.1', 9996), MockHandler)
print("Mock server running on http://127.0.0.1:9996")
httpd.serve_forever()

