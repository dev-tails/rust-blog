use std::fs;
use std::path::Path;
use std::ffi::OsStr;

use tokio::net::TcpListener;
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufStream};

use regex::Regex;

use markdown::markdown_to_html;


#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or(String::from("8000"));

    let bind_address = format!("[::]:{port}");

    let listener = TcpListener::bind(bind_address).await.unwrap();

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

                // GET /hello-world.html HTTP1.1
                let re = Regex::new(r"^(.*) (.*) (.*)").unwrap();
                let caps = re.captures(&request_line).unwrap();
                let pathname = caps.get(2).map_or("", |m| m.as_str());

                let mut filename = "index.html";
                if pathname != "/" {
                    filename = &pathname[1..];
                }

                let mut contents = String::new();

                let full_file_path_string = format!("{}{}", "posts/", filename);
                let path = Path::new(&full_file_path_string);
                if path.exists() {
                    contents = fs::read_to_string(&path).unwrap();
                }

                if path.extension().and_then(OsStr::to_str) == Some("md") {
                    contents = markdown_to_html(contents);
                }

                let content_len = contents.len();

                let formatted = format!("HTTP/1.1 200 OK\r\nContent-Length: {content_len}\r\nConnection: Keep-Alive\r\n\r\n{contents}");
                stream.write(formatted.as_bytes()).await.unwrap();
                stream.flush().await.unwrap();
            }
        });
    }
}