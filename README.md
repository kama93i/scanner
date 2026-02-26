# IP:PORT Scanner 
Port scanner built with rust and the tokio runtime. By default scans all 65535 ports.

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

### Installation
Clone repository and compile (make sure you have cargo installed)
```
git clone https://github.com/kama93i/scanner.git && cd scanner
cargo build --release
```


# TODO
- Vervose option for when piping stdout
- Stealth scan with pnet. Send SYN packet but dont establish TCP connection to not create logs on the victim machine.
