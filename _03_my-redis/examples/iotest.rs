use tokio::fs::File;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Tokio 中的 I/O 操作和 std 在使用方式上几无区别，最大的区别就是前者是异步的
#[tokio::main]
async fn main() -> io::Result<()> {
    println!("tokio I/O");

    println!("\n async fn read");
    // 打开文件
    let mut f = File::open("hello.txt").await?;
    let mut buffer = [0; 10];
    // 读取文件内容
    // 当 read 返回 Ok(0) 时，意味着字节流( stream )已经关闭
    let read_len = f.read(&mut buffer[..]).await?;
    println!("The bytes:{:?}", &buffer[..read_len]);
    println!("The String:{:?}", String::from_utf8(buffer[..read_len].to_vec()).unwrap());

    println!("\n async fn read_to_end");
    let mut buffer = Vec::new();
    let rs = f.read_to_end(&mut buffer).await?;
    println!("The len {} buffer lens {:?}", rs, buffer.len());
    println!(
        "==============================\n{:?}\n===============",
        String::from_utf8(buffer)
    );

    println!("\n async fn write");
    let mut f = File::create("world.txt").await?;
    // 必须await,否则线程结束后，内容未写完成
    f.write(b"Hello world").await?;

    println!("\n 实用函数");
    // tokio::io::copy 异步的将读取器( reader )中的内容拷贝到写入器( writer )中
    let mut reader: &[u8] = b"hello";
    // 创建文件
    let mut file = File::create("foo.txt").await?;
    io::copy(&mut reader, &mut file).await?;

    println!("\n 回声服务( Echo )");



    // 返回
    Ok(())
}
