# EchoServer

A simple tool to echo received payloads.

``` sh
echoserver 0.1.0
HTTP server that echoes request details

USAGE:
    echoserver [FLAGS] [OPTIONS]

FLAGS:
    -h, --help         Prints help information
    -m, --mask-auth    Mask the Authorization header value with "***" in the response
    -q, --quiet        Return a simple 200 Success instead of the full JSON response
    -V, --version      Prints version information

OPTIONS:
    -i, --ip <ip>        IP address to bind the server to [default: 127.0.0.1]
    -p, --port <port>    Port to bind the server to [default: 3000]
```

``` sh
echoserver -q
Server running on http://127.0.0.1:3000
```

``` sh
curl -X GET http://localhost:3000/foo/bar/baz
```


```
# Logged server-side
Response:                       
{                               
  "body": null,                 
  "endpoint": "/foo/bar/baz",
  "headers": {                  
    "accept": "*/*",
    "host": "localhost:3000",
    "user-agent": "curl/7.81.0"
  },                            
  "method": "GET"
}                               

```

``` sh
curl -X POST http://localhost:3000/foo/bar/baz -d '{"key":"value"}' -H "Content-Type: application/json"
```

```
# Logged server-side
Response:                       
{                               
  "body": "{\"key\":\"value\"}", 
  "endpoint": "/foo/bar/baz",
  "headers": {                  
    "accept": "*/*",
    "content-length": "15",
    "content-type": "application/json",
    "host": "localhost:3000",
    "user-agent": "curl/7.81.0"
  },                            
  "method": "POST"
} 
```

