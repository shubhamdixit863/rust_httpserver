use std::fmt::format;
use std::io::{BufRead, BufReader, Write};
#[allow(unused_imports)]
use std::net::TcpListener;
use std::net::TcpStream;
use std::{env, thread};
use std::fs;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let user_agent = http_request.iter()
        .find(|&header| header.starts_with("User-Agent: "))
        .map(|header| header.trim_start_matches("User-Agent: "))
        .unwrap_or("User-Agent not found");
    let collection: Vec<&str> = http_request[0].split(" ").collect();
    // Split this route too
    let route_part:Vec<&str>=collection[1].split("/").collect();
    if route_part[0]=="" &&  route_part[1]==""{
        let response = "HTTP/1.1 200 OK\r\n\r\n";

        stream.write_all(response.as_bytes()).unwrap();
    }else if route_part[0]=="" && route_part[1]=="echo"   {
        // send the body
        let response = format!(
            "HTTP/1.1 200 OK\r\n\
        Content-Type: text/plain\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\r\n\
        {}",
            route_part[2].len(),
            route_part[2]
        );
        stream.write_all(response.as_bytes()).unwrap();

    } else if route_part[0]=="" && route_part[1]=="user-agent"{
        let response = format!(
            "HTTP/1.1 200 OK\r\n\
        Content-Type: text/plain\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\r\n\
        {}",
            user_agent.len(),
            user_agent
        );
        stream.write_all(response.as_bytes()).unwrap();


    }else if route_part[0]=="" && route_part[1]=="files" {
        let env_args: Vec<String> = env::args().collect();
        let mut dir = env_args[2].clone();
        dir.push_str(&route_part[2]);
        let file = fs::read(dir);

        // Read the file whose name is route_part[1]


        match file {
            Ok(_contents)=>{
                let response = format!(
                    "HTTP/1.1 200 OK\r\n\
         Content-Type: application/octet-stream\r\n\
        Content-Length:  {}\r\n\
        Connection: close\r\n\r\n\
        {:?}",
                    _contents.len(),
                    String::from_utf8(_contents).expect("file content")
                );
                stream.write_all(response.as_bytes()).unwrap();

            },
            Err(_e)=>{
                println!("{:?}",_e);
                let response = format!(
                    "HTTP/1.1 404 Not Found\r\n\
        Content-Type: text/plain\r\n\
        Content-Length:  {}\r\n\
        Connection: close\r\n\r\n\
        {}",
                    0,
                    ""
                );
                stream.write_all(response.as_bytes()).unwrap();


            }
        }






    }


    else {
        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    }


}
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                thread::spawn( || {
                    handle_connection(_stream)
                });



            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
