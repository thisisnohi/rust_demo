use mini_redis::{client, Result};

/// .await 只能在 async 函数中使用，如果是以前的 fn main，那它内部是无法直接使用 async 函数的！这个会极大的限制了我们的使用场景
///  异步运行时本身需要初始化
#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello Tokio");

    // 建立与Mini-redis服务器的连接
    let mut client = client::connect("127.0.0.1:6379").await?;

    // 设置 key: "hello" 和 值 : "world
    client.set("hello", "world".into()).await?;

    // 获取 key=“hello"的值
    let result = client.get("hello").await?;
    println!("从服务器端获取到的结果={:?}", result);

    let result = client.get("key1").await?;
    println!("key1={:?}", result);

    println!("\n 异步使用");
    let result = client.get("hello");

    Ok(())
}
