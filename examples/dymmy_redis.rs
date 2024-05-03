use std::net::SocketAddr;

/// Redis Server
use anyhow::Result;
use tokio::{io, net::TcpListener};

/// Redis监听地址
const REDIS_ADDR: &str = "0.0.0.0:6379";

/// Redis缓冲区大小
const REDIS_BUF_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 监听6379端口
    let listener = TcpListener::bind(REDIS_ADDR).await?;

    tracing::info!("DummyRedis liistening on: {}", REDIS_ADDR);

    loop {
        // 获取TcpStream和客户端地址
        let (stream, client_addr) = listener.accept().await?;

        tracing::info!("Accepted connection from: {:?}", client_addr);

        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(stream, client_addr).await {
                tracing::warn!("Error processing connection from {:?}: {}", client_addr, e);
            }
        });
    }
}

/// 处理Redis链接
async fn process_redis_conn(stream: tokio::net::TcpStream, client_addr: SocketAddr) -> Result<()> {
    // 读取缓冲区
    let mut buf = [0u8; REDIS_BUF_SIZE];

    loop {
        // 先判断流是否可读
        stream.readable().await?;

        // 如果可读 尝试读取内容
        match stream.try_read(&mut buf) {
            Ok(0) => {
                // 如果是0 则表示客户端断开连接 EOF
                break;
            }
            Ok(n) => {
                // 正常读取到内容
                tracing::info!("Received {} bytes from client", n);

                // 写入成功响应到流 这里是标准的OK协议
                stream.try_write(b"+OK\r\n").unwrap();
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // 阻塞请求 ？？？ 这个不理解
                continue;
            }
            Err(e) => {
                // 其他异常直接抛出
                return Err(e.into());
            }
        }
    }

    tracing::info!("Client {:?} closed connection", client_addr);

    Ok(())
}
