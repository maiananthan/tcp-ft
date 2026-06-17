# Design

## Design considerations

- Transmission
    - Handshake communication before sending the file data with compressed file size and checksum
    - file compression with gzip compression mechanism before transferring over network
    - CRC checksum with bzip2 algorithm used for integrity check
    - waiting for acknowledgement after sending the full file data and marking the transfer success

- Reception
    - Receive transfer request with compressed file data size and CRC checksum
    - Handshake acknowledgement after connection establishment
    - receiving file data till it reaches file size
    - verify CRC checksum and send acknowledgement back to transmitter
    - De-compress the file received to actual file

<!-- end of file -->
