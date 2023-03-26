use clap::Parser;
use std::io::prelude::*;
use std::net::TcpListener;

/// 启动一个临时的http服务器并发送一次字符串。
#[derive(Parser)]
struct Cli {
    /// 服务指定的端口号
    port: u16,
    /// 服务所发送的内容
    str: String,
}

#[macro_use] extern crate lazy_static;
lazy_static! {
  static ref ARGS: Cli = Cli::parse();
}

const CRLF: &str = "\r\n";
fn main() {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", ARGS.port)).unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer= [0; 4096];
        stream.read(&mut buffer).unwrap();
    
        let contents = &ARGS.str;
        let status = format!("HTTP/1.1 200 OK{}", CRLF);
        let content_type = format!("Content-Type: text/html;charset=utf-8{}", CRLF);
        let server = format!("Server: SendStr{}", CRLF);
        let content_length = format!("Content-Length: {}{}", contents.as_bytes().len(), CRLF);
        let response = format!(
            "{0}{1}{2}{3}{4}{5}",
            status, server, content_type, content_length, CRLF, contents
        );
    
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        break;
    }
}