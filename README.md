# rust study - imitation httpie

## useage

### how to send a http get request
```bash
$ httpie get http://httpbin.org/get

HTTP/1.1 200 OK

date: "Sat, 19 Nov 2022 04:49:11 GMT"
content-type: "application/json"
content-length: "221"
connection: "keep-alive"
server: "gunicorn/19.9.0"
access-control-allow-origin: "*"
access-control-allow-credentials: "true"


{
  "args": {},
  "headers": {
    "Accept": "*/*",
    "Host": "httpbin.org",
    "X-Amzn-Trace-Id": "Root=1-63786047-3d55693c2b53f9ab115f1b6a"
  },
  "origin": "139.226.86.225",
  "url": "http://httpbin.org/get"
}
```

### how to send a http post request
```bash
$ httpie post http://httpbin.org/post name=zhangsan age=19

HTTP/1.1 200 OK

date: "Sat, 19 Nov 2022 04:48:34 GMT"
content-type: "application/json"
content-length: "435"
connection: "keep-alive"
server: "gunicorn/19.9.0"
access-control-allow-origin: "*"
access-control-allow-credentials: "true"


{
  "args": {},
  "data": "{\"name\":\"zhangsan\",\"age\":\"19\"}",
  "files": {},
  "form": {},
  "headers": {
    "Accept": "*/*",
    "Content-Length": "30",
    "Content-Type": "application/json",
    "Host": "httpbin.org",
    "X-Amzn-Trace-Id": "Root=1-63786022-27ba18197bc7dc287c092618"
  },
  "json": {
    "age": "19",
    "name": "zhangsan"
  },
  "origin": "139.226.86.225",
  "url": "http://httpbin.org/post"
}
```