use std::{
    io::{Read, Write},
    thread::sleep,
};

use flate2::Compression;

use crate::common::get_value;

pub fn send_file(recv_addr: String, file_path: String) -> bool {
    println!("recv_addr: {}, file_path: {}", recv_addr, file_path);

    let mut stream = std::net::TcpStream::connect(recv_addr);

    match stream {
        Ok(mut st) => {
            println!("connection success. sending file data...");

            // = = = = = = =
            // Open the file
            // = = = = = = =
            let mut file: std::fs::File;
            match std::fs::File::open(file_path.clone()) {
                Ok(f) => {
                    file = f;
                }
                Err(e) => {
                    println!("error opening the file | err: {}", e);
                    return false;
                }
            }

            // = = = = = = =
            // perform compression
            // = = = = = = =
            let mut ip_reader = std::io::BufReader::new(file);
            let mut op: std::fs::File;
            let mut op_reader = std::fs::File::create("compress.gz");
            match op_reader {
                Ok(reader) => {
                    op = reader;
                }
                Err(e) => {
                    println!("output file creation failed | err: {}", e);
                    return false;
                }
            }
            let mut encoder = flate2::write::GzEncoder::new(&op, Compression::default());
            println!("running file compression before file transfer");
            match std::io::copy(&mut ip_reader, &mut encoder) {
                Ok(size) => {
                    println!("size copied | size: {}", size);
                }
                Err(e) => {
                    println!("error coping to encoder | err: {}", e);
                    return false;
                }
            }
            match encoder.finish() {
                Ok(_) => {
                    println!("file compression completed");
                }
                Err(e) => {
                    println!("error converting to output | err: {}", e);
                }
            }

            // = = = = = = =
            // pack the details into request
            //      checksum of the file (8 bytes), file size (8 bytes)
            // = = = = = = =
            let mut buff: Vec<u8> = Vec::new();

            let op_cs: u64;
            match crc_fast::checksum_file(crc_fast::CrcAlgorithm::Crc32Bzip2, "compress.gz", None) {
                Ok(_checksum) => {
                    op_cs = _checksum;
                }
                Err(e) => {
                    println!("error getting checksum | err: {}", e);
                    return false;
                }
            }
            let output_size: u64;
            match op.metadata() {
                Ok(us) => {
                    output_size = us.len();
                }
                Err(e) => {
                    println!("error getting metadata: {}", e);
                    return false;
                }
            }
            println!(
                "crc checksum: 0x{:x} : {}, output size: {}",
                op_cs, op_cs, output_size
            );

            buff.push(0x01);
            // println!("buff: {:?}", buff);
            buff.push(0x02);
            // println!("buff: {:?}", buff);
            for i in op_cs.to_be_bytes() {
                buff.push(i);
            }
            // println!("buff: {:?}", buff);
            for i in output_size.to_be_bytes() {
                buff.push(i);
            }
            // println!("buff: {:?}", buff);

            // = = = = = = =
            // send request
            // = = = = = = =
            match st.write(&buff) {
                Ok(us) => {
                    if us > 0 {
                        println!("request sent successfully. sending data...");
                    } else {
                        println!("unable to send request");
                        return false;
                    }
                }
                Err(e) => {
                    println!("error sending request | err: {}", e);
                    return false;
                }
            }

            // = = = = = = =
            // wait for request ack
            // = = = = = = =
            match st.flush() {
                Ok(_) => {}
                Err(e) => {
                    println!("error flushing | err: {}", e);
                }
            }
            buff = vec![0u8; 10];
            match st.read(&mut buff) {
                Ok(us) => {
                    // println!("buff: {:?}", buff);
                    if us > 0 {
                        if (buff[0] == 0x03) && (buff[1] == 0x04) {
                            println!("ack received. proceed sending the file");
                        } else {
                            println!("wrong ack");
                            return false;
                        }
                    } else {
                        println!("ack not received");
                        return false;
                    }
                }
                Err(e) => {
                    println!("error read from network | err: {}", e);
                    return false;
                }
            }

            // = = = = = = =
            // send the data
            // = = = = = = =
            match std::fs::OpenOptions::new().read(true).open("compress.gz") {
                Ok(f) => {
                    let mut buff_read = std::io::BufReader::new(f);

                    loop {
                        buff = vec![0u8; crate::common::DATA_SIZE];
                        match buff_read.read(&mut buff) {
                            Ok(us) => {
                                println!("read from file: usize: {}", us);
                                // println!("write buff: {:?}", buff);
                                if us > 0 {
                                    match st.write(&buff[0..us]) {
                                        Ok(us) => {
                                            println!("write to network: us: {}", us);
                                        }
                                        Err(e) => {
                                            println!("error writing to network | err: {}", e);
                                            break;
                                        }
                                    }
                                } else {
                                    println!(
                                        "file have reached the end. proceeding to checksum validation..."
                                    );
                                    break;
                                }
                            }
                            Err(e) => {
                                println!("error reading file | err: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("unable to open file | err: {}", e);
                    return false;
                }
            }
            // = = = = = = =
            // recv data completion info
            // = = = = = = =
            println!("waiting for recv comp info");
            buff = vec![0u8; 10];
            match st.flush() {
                Ok(_) => {}
                Err(e) => {
                    println!("error flushing | err: {}", e);
                }
            }

            loop {
                match st.read(&mut buff) {
                    Ok(us) => {
                        println!("buff: {:?}", buff);
                        if us > 0 {
                            if (buff[0] == 0x05) && (buff[1] == 0x06) {
                                println!("recv data completion info");
                                break;
                            } else {
                                println!("wrong recv data info");
                                // return false;
                            }
                        } else {
                            println!("ack not received");
                            return false;
                        }
                    }
                    Err(e) => {
                        println!("error read from network | err: {}", e);
                        return false;
                    }
                }
            }
        }
        Err(e) => {
            println!("error connecting to server | err: {}", e);
        }
    }
    return true;
}
