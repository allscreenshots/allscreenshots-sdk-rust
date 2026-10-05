#!/usr/bin/env python3
"""Local browser demo. Each request runs the native language SDK worker."""
import json, os, secrets, subprocess, tempfile
from pathlib import Path
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import urlparse

ROOT=Path(__file__).resolve().parent.parent
TOKEN=secrets.token_hex(24)
CONFIG=json.loads((ROOT/'sdk-build.json').read_text())
OUTPUT=Path(tempfile.mkdtemp(prefix='allscreenshots-demo-'))

def command():
    run=list(CONFIG['run'])
    if CONFIG.get('javaClasspath'):
        run[2]=str(ROOT/'target/classes')+os.pathsep+(ROOT/'target/classpath.txt').read_text().strip()
    return run

class Handler(BaseHTTPRequestHandler):
    def local_host(self):
        return self.headers.get('Host') in {f'127.0.0.1:{self.server.server_port}',f'localhost:{self.server.server_port}'}
    def send(self,code,body,content='application/json'):
        self.send_response(code);self.send_header('Content-Type',content);self.send_header('Content-Length',str(len(body)));self.send_header('X-Content-Type-Options','nosniff');self.end_headers();self.wfile.write(body)
    def do_GET(self):
        if not self.local_host():self.send(403,b'{}');return
        if self.path=='/':
            html=(ROOT/'sample-app/app.html').read_text().replace('__TOKEN__',TOKEN).replace('__LANGUAGE__',CONFIG['language'].title())
            self.send(200,html.encode(),'text/html; charset=utf-8')
        elif self.path.startswith('/preview/'):
            name=self.path.removeprefix('/preview/')
            if len(name)==32 and all(c in '0123456789abcdef' for c in name) and (OUTPUT/(name+'.png')).exists():
                self.send(200,(OUTPUT/(name+'.png')).read_bytes(),'image/png')
            else:self.send(404,b'{}')
        else:self.send(404,b'{}')
    def do_POST(self):
        if not self.local_host():self.send(403,b'{}');return
        mode=self.path.removeprefix('/api/')
        origin=self.headers.get('Origin')
        expected=f'http://{self.headers.get("Host")}'
        if (origin and origin!=expected) or self.headers.get('X-Demo-Token')!=TOKEN:
            self.send(403,b'{"error":"Open the local demo page to make requests."}');return
        if mode not in ['quota','sync','async']:
            self.send(404,b'{}');return
        try:
            length=int(self.headers.get('Content-Length','0'))
            if not 0<=length<=8192:raise ValueError('Request too large')
            body=json.loads(self.rfile.read(length))
            url=body.get('url','https://example.com')
            if mode!='quota' and urlparse(url).scheme not in ['http','https']:raise ValueError('Enter an HTTP or HTTPS URL')
            name=secrets.token_hex(16)
            env=os.environ|{'ALLSCREENSHOTS_URL':url,'ALLSCREENSHOTS_OUTPUT':str(OUTPUT/(name+'.png')),'ALLSCREENSHOTS_IDEMPOTENCY_KEY':name}
            process=subprocess.run(command()+[mode],cwd=ROOT/CONFIG.get('runCwd','.'),env=env,capture_output=True,text=True,timeout=240)
            if process.returncode:raise ValueError('The SDK request failed. Check your API key, quota, and API availability.')
            result=json.loads(process.stdout.strip().splitlines()[-1])
            self.send(200,json.dumps(result if mode=='quota' else {'id':name}).encode())
        except (ValueError,subprocess.TimeoutExpired):self.send(400,b'{"error":"The SDK request failed. Check your API key, quota, URL, and API availability."}')
    def log_message(self,*args):pass

if __name__=='__main__':
    if not os.getenv('ALLSCREENSHOTS_API_KEY'):raise SystemExit('Set ALLSCREENSHOTS_API_KEY before starting the demo.')
    port=int(os.getenv('PORT','7070'));print(f'Open http://127.0.0.1:{port}',flush=True)
    ThreadingHTTPServer(('127.0.0.1',port),Handler).serve_forever()
