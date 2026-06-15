use std::io::{Read, Write};

fn handle_client(mut stream: std::net::TcpStream) -> bool {
    // read the data and wait for requests
    // after request received, read for actual data
    let mut buff = vec![0u8; crate::common::DATA_SIZE];

    let mut file: std::fs::File;

    // create file
    // if file exists, delete and create new
    match std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .truncate(false)
        .open("a.txt")
    {
        Ok(f) => {
            file = f;
        }
        Err(e) => {
            println!("error opening file | err: {}", e);
            return false;
        }
    }

    let mut buff_writer = std::io::BufWriter::new(file);

    loop {
        let data = stream.read(&mut buff);

        match data {
            Ok(s) => {
                if s > 0 {
                    match crate::common::get_state(buff[0]) {
                        crate::common::CommState::Request => {
                            println!("request received. start receiving data");
                        }
                        crate::common::CommState::DataTransfer => {
                            match buff_writer.write(&buff[1..s]) {
                                Err(e) => {
                                    println!("unable to write to file | err: {}", e);
                                }
                                _ => {}
                            }
                        }
                        crate::common::CommState::Checksum => {
                            println!("checksum");
                        }
                        crate::common::CommState::Unknown => {
                            println!("unknown data");
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
                        true => {}
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
