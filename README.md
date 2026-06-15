# tcp-ft

- TCP based file transfer CLI utility between two systems connected to the network.

- Supports:
    1. cross-platform compactible
    2. larger file transfer support (upto 16 GiB)
    3. on-the-fly data compression for lesser bandwidth consumption
    4. integrity check using checksum validation

## build and run

- install rust compiler using `rustup` tool

- run below build command. output executable will be available in `./target/release/tcp-ft`

```bash
cargo build --release
```

- available options:

- `send`: send file
    - `--recv-addr <ADDRESS>`: receiver address with port (example: `192.168.1.1:8080`)
    - `--file <FILE_PATH>`: file path to send (example: `a.txt`)

- `recv`: receive file from sender

<!-- end of file -->
