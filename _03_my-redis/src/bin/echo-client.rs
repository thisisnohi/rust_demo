use io::Error;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// 回声服务
/// 运行方式：cargo run --bin echo-server-copy
#[tokio::main]
async fn main() -> io::Result<()> {
    println!("回声服务 客户端");

    let stream = TcpStream::connect("127.0.0.1:6142").await?;
    // 分离读写器
    // io::split 可以用于任何同时实现了 AsyncRead 和 AsyncWrite 的值
    let (mut rd, mut wr) = io::split(stream);

    // 创建异步任务，在后台写入数据
    tokio::spawn(async move {
        wr.write_all(b"hello\r\n").await?;
        wr.write_all(b"world\r\n").await?;
        wr.write_all(b"3:....\r\n").await?;

        // 有时，我们需要给予Rust一些类型暗示，它才能正确的推导出类型
        Ok::<_, Error>(())
    });

    // 读取数据
    // 缓冲区是一个 Vec 动态数组，它的数据是存储在堆上，而不是栈上(若改成 let mut buf = [0; 1024];，则存储在栈上)
    let mut buf = vec![0; 128];
    loop {
        let n = rd.read(&mut buf).await?;
        if n == 0 {
            println!("没有读取到任何内容");
            break;
        }
        println!("{:?}", &buf[..n]);
        println!("{:?}", String::from_utf8(buf[..n].to_vec()).unwrap());
        break;
    }

    Ok(())
}
