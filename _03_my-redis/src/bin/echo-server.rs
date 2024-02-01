use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

/// 回声服务
/// 运行方式：cargo run --bin echo-server-copy
#[tokio::main]
async fn main() -> io::Result<()> {
    println!("回声服务");

    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut stream, addres) = listener.accept().await?;
        println!("Accepted connection from: {}", addres);

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                // 方式1： 手工处理数据
                // match stream.read(&mut buf).await {
                //     // 返回值 `Ok(0)` 说明对端已经关闭
                //     Ok(0) => return,
                //     Ok(n) => {
                //         // Copy the data back to socket
                //         // 将数据拷贝回 socket 中
                //         if stream.write_all(&buf[..n]).await.is_err() {
                //             eprintln!("非预期错误，由于我们这里无需再做什么，因此直接停止处理");
                //             // 非预期错误，由于我们这里无需再做什么，因此直接停止处理
                //             return;
                //         };
                //     }
                //     Err(_) => {
                //         return;
                //     }
                // }

                // 方式2： 使用 `copy` 函数
                let (mut rt, mut wt) = stream.split();
                // 拷贝数据
                if io::copy(&mut rt, &mut wt).await.is_err() {
                    eprintln!("failed to copy")
                }
            }
        });
    }

    println!("回声服务结束!");

    Ok(())
}
