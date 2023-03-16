use std::fs;
use std::path::Path;

use tokio::net::TcpListener;
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufStream};

use regex::Regex;


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("[::]:8000").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut stream = BufStream::new(stream);

            // let val = "Hello World";
            // let content_len = val.len();
            // let formatted = format!("HTTP/1.1 200 OK Content-Length: {content_len}\r\nConnection: Keep-Alive\r\n\r\n{val}\r\n\r\n");

            loop {
                let mut line = String::new();
                
                let mut request_line = String::new();

                loop {
                    stream.read_line(&mut line).await.unwrap();

                    if request_line == "" {
                        request_line = String::from(&line);
                    }

                    if line == "\n" || line == "\r\n" {
                        break;
                    }
                    if line == "" {
                        return;
                    }

                    line.truncate(0);
                }

                // let re = Regex::new(r"^(.*) (.*) (.*)").unwrap();
                // let caps = re.captures(&request_line).unwrap();
                // let pathname = caps.get(2).map_or("", |m| m.as_str());

                // let mut filename = "index.html";
                // if pathname != "/" {
                //     filename = &pathname[1..];
                // }

                let mut contents = String::new();

                // let full_file_path = format!("{}{}", "posts/", filename);
                // if Path::new(&full_file_path).exists() {
                //     contents = fs::read_to_string(&full_file_path).unwrap();
                // }

                let content_len = contents.len();

                let formatted = format!("HTTP/1.1 200 OK\r\nContent-Length: {content_len}\r\nConnection: Keep-Alive\r\n\r\n{contents}");
                stream.write(formatted.as_bytes()).await.unwrap();
                stream.flush().await.unwrap();
            }
        });
    }
}