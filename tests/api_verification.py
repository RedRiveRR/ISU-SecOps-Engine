import urllib.request
import json
import time

API_URL = "http://127.0.0.1:8085/api/scan"
MOCK_SERVER = "http://127.0.0.1:9996"

def test_api():
    print(f"[*] Starting scan against {MOCK_SERVER}...")
    payload = {
        "url": MOCK_SERVER,
        "threads": 5,
        "auto_wordlist": True,
        "auto_threads": True,
        "crawler": True,
        "depth": 1
    }
    
    try:
        req = urllib.request.Request(API_URL, data=json.dumps(payload).encode('utf-8'), 
                                     headers={'Content-Type': 'application/json'}, method='POST')
        with urllib.request.urlopen(req) as response:
            data = json.loads(response.read().decode('utf-8'))
            stream_id = data.get("stream_id")
            print(f"[*] Scan started! Stream ID: {stream_id}")
        
        # Test SSE Stream
        print(f"[*] Listening to SSE stream for {stream_id}...")
        sse_url = f"{API_URL}/stream/{stream_id}"
        
        req_sse = urllib.request.Request(sse_url)
        with urllib.request.urlopen(req_sse) as sse_response:
            count = 0
            while True:
                line = sse_response.readline()
                if not line:
                    break
                decoded_line = line.decode('utf-8').strip()
                if decoded_line.startswith("data:"):
                    event_data = json.loads(decoded_line[5:])
                    print(f"[EVENT] {event_data.get('event','?')}: {str(event_data)[:80]}...")
                    count += 1
                    if event_data.get("event") == "Finished" or count > 30:
                        break
        
        print("[+] API and SSE test completed successfully!")
        
    except Exception as e:
        print(f"[!] Test failed: {e}")

if __name__ == "__main__":
    test_api()
