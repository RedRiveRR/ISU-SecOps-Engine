from http.server import HTTPServer, BaseHTTPRequestHandler

class MockHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header('Content-type', 'text/html')
        self.end_headers()
        
        if self.path == '/':
            self.wfile.write(b"<html><body><a href='/page1'>Page 1</a><a href='/page2'>Page 2</a></body></html>")
        elif self.path == '/page1':
            self.wfile.write(b"<html><body><a href='/page1/subpage'>Subpage</a></body></html>")
        elif self.path == '/page1/subpage':
            self.wfile.write(b"<html><body><a href='/found_by_crawler'>Secret Link</a></body></html>")
        else:
            self.wfile.write(b"<html><body>Found!</body></html>")

httpd = HTTPServer(('127.0.0.1', 9999), MockHandler)
print("Mock server running on http://127.0.0.1:9999")
httpd.serve_forever()
