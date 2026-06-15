use std::{
    io::{Read, Write},
    thread::sleep,
};

use crate::common::get_value;

pub fn send_file(recv_addr: String, file_path: String) -> bool {
    println!("recv_addr: {}, file_path: {}", recv_addr, file_path);

    let mut stream = std::net::TcpStream::connect(recv_addr);

    match stream {
        Ok(mut st) => {
            println!("connection success. sending file data...");

            // send request
            let mut buff = vec![0u8; crate::common::DATA_SIZE];
            buff[0] = crate::common::get_value(crate::common::CommState::Request);

            match st.write(&buff) {
                Ok(us) => {
                    if us == crate::common::DATA_SIZE {
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

            // open and read the file
            // write into tcp stream till EOF
            match std::fs::OpenOptions::new().read(true).open(&file_path) {
                Ok(f) => {
                    let mut buff_read = std::io::BufReader::new(f);
                    buff[0] = crate::common::get_value(crate::common::CommState::DataTransfer);

                    loop {
                        match buff_read.read(&mut buff[1..]) {
                            Ok(us) => {
                                if us > 0 {
                                    match st.write(&buff[0..us]) {
                                        Ok(_) => {}
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

            // send checksum
        }
        Err(e) => {
            println!("error connecting to server | err: {}", e);
        }
    }
    return true;
}
