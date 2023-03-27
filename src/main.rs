use clap::Parser;
use std::io::prelude::*;
use std::net::TcpListener;

/// 启动一个临时的http服务器并发送一次字符串，
/// 当接收到POST请求时将会输出Content。
#[derive(Parser)]
struct Cli {
    /// 服务指定的端口号
    port: u16,
    /// 服务所发送的内容
    str: String,
}

fn main() {
    let args = Cli::parse();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port)).unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 4096];
        stream.read(&mut buffer).unwrap();

        let req = String::from_utf8_lossy(&buffer);
        for st in req.split("\r\n\r\n") {
            if st.starts_with("GET") {
                break;
            }
            if !st.starts_with("POST") {
                println!("{}", st.trim())
            }
        }

        let contents = &args.str;
        let status = "HTTP/1.1 200 OK\r\n";
        let content_type = "Content-Type: text/html;charset=utf-8\r\n";
        let server = "Server: SendStr\r\n";
        let content_length = format!("Content-Length: {}\r\n", contents.as_bytes().len());
        let response = format!(
            "{0}{1}{2}{3}\r\n{4}",
            status, server, content_type, content_length, contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        break;
    }
}