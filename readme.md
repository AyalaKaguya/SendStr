# SendStr

一个简单的HTTP服务器用于在命令行跨主机传输单行字符串。

## 使用方法

```document
启动一个临时的http服务器并发送一次字符串， 当接收到POST请
求时将会输出Content。

Usage: sendstr.exe <PORT> <STR>

Arguments:
  <PORT>  服务指定的端口号
  <STR>   服务所发送的内容

Options:
  -h, --help  Print help
```

## 示例

在8901端口上启动一个简单的http服务器，请求一次即关闭。

```cmd
sendstr 8901 "Hello,World"
```

可接受GET请求和POST请求，可以在浏览器内访问，当接受POST请求时将会输出内容。

POST请求示例 `curl -d 'Data' -X POST 127.0.0.1:8901`

- 将会向服务端发送字符串`Data`，并在服务端输出
- 接收到服务端返回的字符串`Hello,World`并输出

GET请求示例 `curl 127.0.0.1:8901`

- 接收到服务端返回的字符串`Hello,World`并输出

## 一些高级玩法

```cmd
sendstr 2345 "<h1>Hello,World</h1><p>test</p>"
```

然后在浏览器里访问`127.0.0.1:2345`...
