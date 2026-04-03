from http.server import HTTPServer, BaseHTTPRequestHandler

class MockHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path.startswith('/waf'):
            self.send_response(429)
            self.end_headers()
            self.wfile.write(b"Too Many Requests")
            return
            
        self.send_response(200)
        self.send_header('Content-type', 'text/html')
        self.end_headers()
        
        if self.path == '/':
            self.wfile.write(b"<html><head><title>Home</title></head><body><a href='/page1'>Page 1</a><a href='/page2'>Page 2</a></body></html>")
        elif self.path == '/page1':
            self.wfile.write(b"<html><head><title>Page One</title></head><body><a href='/page1/subpage'>Subpage</a></body></html>")
        elif self.path == '/page1/subpage':
            self.wfile.write(b"<html><head><title>Deep Subpage</title></head><body><a href='/found_by_crawler'>Secret Link</a><a href='/waf_test'>WAF Test</a></body></html>")
        elif self.path.startswith('/waf'):
            self.send_response(429)
            self.end_headers()
            self.wfile.write(b"Too Many Requests")
        else:
            self.wfile.write(b"<html><head><title>Secret Found!</title></head><body>Found!</body></html>")

httpd = HTTPServer(('127.0.0.1', 9998), MockHandler)
print("Mock server running on http://127.0.0.1:9998")
httpd.serve_forever()
