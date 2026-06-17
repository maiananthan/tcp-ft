use std::io::{Read, Write};

fn vec_to_u64(bytes: &[u8]) -> u64 {
    let mut buf: [u8; 8] = [0; 8];
    for i in 0..8 {
        buf[i] = bytes[i];
    }
    u64::from_be_bytes(buf)
}

fn handle_client(mut stream: std::net::TcpStream) -> bool {
    // read the data and wait for requests
    // after request received, read for actual data
    let mut buff = vec![0u8; crate::common::DATA_SIZE];

    let mut op_file: std::fs::File;
    let mut t_state = crate::common::TransferState::TransferReq;
    let mut op_checksum: u64 = 0;
    let mut op_size: u64 = 0;
    let mut recv_size: u64 = 0;
    // create file
    // if file exists, delete and create new
    match std::fs::remove_file("recv-compress.gz") {
        Ok(_) => {
            println!("deleted recv-compress.gz");
        }
        Err(e) => {
            println!("failed to delete recv-compress.gz");
        }
    }

    match std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .truncate(false)
        .open("recv-compress.gz")
    {
        Ok(f) => {
            op_file = f;
        }
        Err(e) => {
            println!("error opening file | err: {}", e);
            return false;
        }
    }

    let mut buff_writer = std::io::BufWriter::new(op_file);

    loop {
        let data = stream.read(&mut buff);

        match data {
            Ok(s) => {
                // println!("read from network: s: {}", s);
                // println!("read buffer: {:?}", buff);
                if s > 0 {
                    match t_state {
                        crate::common::TransferState::TransferReq => {
                            // = = = = = = =
                            // read request
                            // = = = = = = =
                            if buff[0] == 0x01 && buff[1] == 0x02 {
                                op_checksum = vec_to_u64(&buff[2..10]);
                                op_size = vec_to_u64(&buff[10..18]);
                            }

                            // = = = = = =
                            // send request ack
                            // = = = = = =
                            let ack = vec![0x03u8, 0x04u8];

                            // println!("write buff: {:?}", ack);
                            match stream.write(&ack) {
                                Ok(us) => {
                                    println!("request ack write");
                                    t_state = crate::common::TransferState::TransferInit;
                                }
                                Err(e) => {
                                    println!("request ack write failed | err: {}", e);
                                    break;
                                }
                            }
                        }
                        crate::common::TransferState::TransferInit => {
                            match buff_writer.write(&buff[0..s]) {
                                Ok(us) => {
                                    recv_size += us as u64;

                                    if recv_size == op_size {
                                        println!("received full file");
                                        match buff_writer.flush() {
                                            Ok(_) => {}
                                            Err(e) => {
                                                println!("flush failed | err: {}", e);
                                            }
                                        }
                                    } else {
                                        continue;
                                    }
                                    println!("size: {} | recv_size: {}", us, recv_size);
                                }
                                Err(e) => {
                                    println!("error writing to compress.gz | err: {}", e);
                                }
                            }

                            // = = = = = = =
                            // run crc check
                            // = = = = = = =

                            match crc_fast::checksum_file(
                                crc_fast::CrcAlgorithm::Crc32Bzip2,
                                "recv-compress.gz",
                                None,
                            ) {
                                Ok(_cs) => {
                                    if _cs != op_checksum {
                                        println!("op_checksum: {}, _cs: {}", op_checksum, _cs);
                                        println!("checksum failed");
                                        return false;
                                    }
                                }
                                Err(e) => {
                                    println!("crc checksum calculation failed | err: {}", e);
                                    return false;
                                }
                            }

                            let mut success = false;
                            loop {
                                buff = Vec::new();
                                buff.push(0x05);
                                buff.push(0x06);
                                println!("buff: {:?}", buff);
                                match stream.write(&buff) {
                                    Ok(us) => {
                                        println!("write transfer comp: {}", us);
                                        success = true;
                                        break;
                                    }
                                    Err(e) => {
                                        println!("unable to write transfer comp");
                                        // return false;
                                    }
                                }
                            }
                            break;
                        }
                        crate::common::TransferState::TransferComp => {}
                        _ => {
                            println!("unhandled state");
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
            Err(e) => {
                println!("data recv failed | err: {}", e);
                break;
            }
        }
    }

    // = = = = = = =
    // uncompress the file
    // = = = = = = =
    match std::fs::remove_file("output.txt") {
        Ok(_) => {
            println!("deleted output.txt");
        }
        Err(e) => {
            println!("failed to delete output.txt");
        }
    }

    let mut o_file;
    match std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("output.txt")
    {
        Ok(f) => {
            o_file = f;
        }
        Err(e) => {
            println!("error opening file | err: {}", e);
            return false;
        }
    }

    let mut ip_file: std::fs::File;
    match std::fs::File::open("recv-compress.gz") {
        Ok(f) => {
            ip_file = f;
        }
        Err(e) => {
            println!("error opening the ip_file | err: {}", e);
            return false;
        }
    }

    let mut ip_reader = std::io::BufReader::new(ip_file);
    let mut dec = flate2::write::GzDecoder::new(o_file);
    match std::io::copy(&mut ip_reader, &mut dec) {
        Ok(size) => {
            println!("size copied | size: {}", size);
        }
        Err(e) => {
            println!("error decoding file | err: {}", e);
            return false;
        }
    }

    match dec.finish() {
        Ok(_) => {}
        Err(e) => {
            println!("error converting | err: {}", e);
            return false;
        }
    }
    return true;
}

pub fn recv_file() {
    // wait for requests
    // if new request comes, approve and recv data

    let listener = std::net::TcpListener::bind("0.0.0.0:8080");

    println!("created listener on port 8080 and waiting for clients...");

    match listener {
        Ok(lt) => {
            for stream in lt.incoming() {
                match stream {
                    Ok(st) => match handle_client(st) {
                        true => {
                            println!("recv full file");
                        }
                        false => {
                            println!("client handling failed");
                        }
                    },
                    Err(e) => {
                        println!("unable to get stream | err: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("unable to get listener | err: {}", e);
        }
    }
}
