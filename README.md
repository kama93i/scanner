# IP:PORT Scanner 
Port scanner build with rust and the tokio runtime. By default scans all 65535 ports.

### Usage:
Simple scan
```
scanner -a 127.0.0.1 -p 0-1280
```

Scan and pipe
```
scanner -a 127.0.0.1 > scan.txt
```
```
// scan.txt
127.0.0.1:22
127.0.0.1:80
```


