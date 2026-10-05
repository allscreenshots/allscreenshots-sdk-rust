#!/usr/bin/env python3
"""Build a root SDK and exercise its real HTTP transport against a local contract fixture."""
import argparse, hashlib, json, os, re, secrets, struct, subprocess, tempfile, threading, zlib
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

ROOT=Path(__file__).resolve().parent.parent
CONFIG=json.loads((ROOT/'sdk-build.json').read_text())

def native_command():
    command=list(CONFIG['run'])
    if CONFIG.get('javaClasspath'):
        command[2]=str(ROOT/'target/classes')+os.pathsep+(ROOT/'target/classpath.txt').read_text().strip()
    return command

def build():
    env=os.environ.copy()
    if CONFIG['language']=='java':
        # Maven otherwise honors an unrelated JAVA_HOME even when mise selects the pinned java on PATH.
        settings=subprocess.check_output(['java','-XshowSettings:properties','-version'],stderr=subprocess.STDOUT,text=True)
        home=re.search(r'^\s*java.home = (.+)$',settings,re.MULTILINE)
        if not home:raise SystemExit('Cannot determine the selected Java runtime')
        env['JAVA_HOME']=home[1].strip()
    if CONFIG['language']=='ruby':env['BUNDLE_FROZEN']='true'
    for index,command in enumerate(CONFIG['build']):
        cwd=ROOT/CONFIG.get('buildCwd',['.']*len(CONFIG['build']))[index]
        subprocess.run(command,cwd=cwd,env=env,check=True)
    if CONFIG['language']=='java':
        expected=json.loads((ROOT/'maven-artifacts.json').read_text())
        actual={}
        for entry in (ROOT/'target/classpath.txt').read_text().strip().split(os.pathsep):
            path=Path(entry);key=str(path).split('/.m2/repository/')[-1]
            actual[key]=hashlib.sha256(path.read_bytes()).hexdigest()
        if actual!=expected:raise SystemExit('Maven dependency checksums differ from maven-artifacts.json')

def worker(mode,env):
    result=subprocess.run(native_command()+[mode],cwd=ROOT/CONFIG.get('runCwd','.'),env=os.environ|env,capture_output=True,text=True,timeout=240)
    return result

def png():
    def chunk(kind,data): return struct.pack('>I',len(data))+kind+data+struct.pack('>I',zlib.crc32(kind+data)&0xffffffff)
    return b'\x89PNG\r\n\x1a\n'+chunk(b'IHDR',struct.pack('>IIBBBBB',2,2,8,2,0,0,0))+chunk(b'IDAT',zlib.compress(b'\0\xff\0\0\0\xff\0'*2))+chunk(b'IEND',b'')
PNG=png()
BANDWIDTH={'limitBytes':1000000,'limitFormatted':'1 MB','usedBytes':len(PNG),'usedFormatted':'1 KB','remainingBytes':999000,'remainingFormatted':'999 KB','percentUsed':0}
QUOTA={'tier':'pro','screenshots':{'limit':100,'used':2,'remaining':98,'percentUsed':2},'bandwidth':BANDWIDTH,'periodEnds':None}

class Fixture(BaseHTTPRequestHandler):
    submissions={}; captures=0; polls={}; lock=threading.Lock()
    def log_message(self,*args):pass
    def reply(self,status,body):
        binary=isinstance(body,bytes);data=body if binary else json.dumps(body).encode()
        self.send_response(status);self.send_header('Content-Type','image/png' if binary else 'application/json');self.send_header('Content-Length',str(len(data)));self.end_headers();self.wfile.write(data)
    def authorized(self):
        if self.headers.get('X-API-Key')!='fixture-key':
            self.reply(401,{'error':{'code':'INVALID_API_KEY','message':'Invalid API key'},'requestId':'fixture'});return False
        return True
    def do_GET(self):
        if not self.authorized():return
        if self.path=='/v1/usage/quota':self.reply(200,QUOTA)
        elif self.path.endswith('/result'):self.reply(200,PNG)
        elif self.path.startswith('/v1/screenshots/jobs/'):
            id=self.path.rsplit('/',1)[-1]
            with self.lock:self.polls[id]=self.polls.get(id,0)+1;count=self.polls[id]
            state='FAILED' if id=='failed' else ('PROCESSING' if count==1 else 'COMPLETED')
            self.reply(200,{'id':id,'status':state,'url':'https://example.com','createdAt':'2026-01-01T00:00:00Z','resultUrl':f'/v1/screenshots/jobs/{id}/result','errorCode':'SELECTOR_NOT_FOUND' if state=='FAILED' else None,'errorMessage':'Selector not found' if state=='FAILED' else None})
        else:self.reply(404,{'error':{'code':'NOT_FOUND','message':'Not found'}})
    def do_POST(self):
        if not self.authorized():return
        body=json.loads(self.rfile.read(int(self.headers['Content-Length'])))
        # Catch SDKs sending renamed wire fields, invalid zero defaults, or missing enum values.
        if body.get('responseType')!='URL' or body.get('format')!='png' or not body.get('url','').startswith('https://') or not self.headers.get('Idempotency-Key'):
            self.reply(400,{'error':{'code':'INVALID_REQUEST','message':'Invalid wire request'}});return
        for name,minimum in [('timeout',1000),('quality',1),('scrollInterval',50)]:
            if name in body and body[name] is not None and body[name]<minimum:
                self.reply(400,{'error':{'code':'INVALID_REQUEST','message':'Invalid default'}});return
        key=(self.path,self.headers['Idempotency-Key'])
        with self.lock:
            if key not in self.submissions:self.submissions[key]=body;type(self).captures+=1
            elif self.submissions[key]!=body:self.reply(409,{'error':{'code':'IDEMPOTENCY_CONFLICT','message':'Changed body'}});return
        if self.path=='/v1/screenshots/async':
            self.reply(202,{'id':'failed' if body['url'].endswith('/failed') else self.headers['Idempotency-Key'],'status':'QUEUED','statusUrl':'/v1/screenshots/jobs/fixture','createdAt':'2026-01-01T00:00:00Z'})
        elif self.path=='/v1/screenshots':
            self.reply(200,{'url':body['url'],'format':'png','contentType':'image/png','width':2,'height':2,'size':len(PNG),'renderTimeMs':10,'cached':False,'resultUrl':f'http://{self.headers["Host"]}/v1/screenshots/captures/fixture/result','storageUrl':None,'expiresAt':None})
        else:self.reply(404,{'error':{'code':'NOT_FOUND','message':'Not found'}})

def fixture_test():
    Fixture.submissions={};Fixture.captures=0;Fixture.polls={}
    server=ThreadingHTTPServer(('127.0.0.1',0),Fixture)
    threading.Thread(target=server.serve_forever,daemon=True).start()
    try:
        with tempfile.TemporaryDirectory(prefix='sdk-transport-') as temp:
            env={'ALLSCREENSHOTS_API_KEY':'fixture-key','ALLSCREENSHOTS_BASE_URL':f'http://127.0.0.1:{server.server_port}','ALLSCREENSHOTS_URL':'https://example.com','ALLSCREENSHOTS_OUTPUT':str(Path(temp)/'capture.png')}
            quota=worker('quota',env)
            assert quota.returncode==0,quota.stderr
            assert json.loads(quota.stdout.strip().splitlines()[-1])['screenshots']['remaining']==98
            for mode in ['sync','async']:
                key=secrets.token_hex(16);settings=env|{'ALLSCREENSHOTS_IDEMPOTENCY_KEY':key}
                for _ in range(2): # Replays must not create an extra capture.
                    capture=worker(mode,settings)
                    assert capture.returncode==0,f'{mode}: {capture.stderr}'
                    assert Path(env['ALLSCREENSHOTS_OUTPUT']).read_bytes()==PNG,'Binary transport corrupted the PNG'
            assert Fixture.captures==2,f'Replays created {Fixture.captures} captures'
            denied=worker('quota',env|{'ALLSCREENSHOTS_API_KEY':'invalid'})
            assert denied.returncode!=0,'401 incorrectly reported as success'
            failed=worker('async',env|{'ALLSCREENSHOTS_URL':'https://example.com/failed','ALLSCREENSHOTS_IDEMPOTENCY_KEY':secrets.token_hex(16)})
            assert failed.returncode!=0,'Failed async job incorrectly reported as success'
        report={'language':CONFIG['language'],'transport':'local HTTP fixture','quota':True,'sync':True,'async':True,'binarySha256':hashlib.sha256(PNG).hexdigest(),'idempotentReplays':True,'unauthorized':True,'failedJob':True}
        (ROOT/'test-results').mkdir(exist_ok=True);(ROOT/'test-results/transport.json').write_text(json.dumps(report,indent=2)+'\n')
        print(f"{CONFIG['language']}: HTTP transport checks passed")
    finally:server.shutdown();server.server_close()

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--build',action='store_true');parser.add_argument('--fixture',action='store_true');args=parser.parse_args()
    if not args.build and not args.fixture:parser.error('Choose --build and/or --fixture')
    if args.build:build()
    if args.fixture:fixture_test()
