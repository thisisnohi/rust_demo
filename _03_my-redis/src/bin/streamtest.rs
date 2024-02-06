
use mini_redis::client;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    println!("# Stream");

    println!("\n## 迭代");
    let vec = vec![1, 2, 3];
    let mut stream = tokio_stream::iter(vec);

    while let Some(item) = stream.next().await {
        println!("{}", item);
    }

    println!("\n## mini-redis 广播");
    tokio::spawn(async { publish().await });

    subscribe().await?;

    println!("DONE");

    Ok(())
}

async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    // 发布一些数据
    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "two".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "five".into()).await?;
    client.publish("numbers", "6".into()).await?;
    client.publish("numbers", "2".into()).await?;
    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;
    // 读完所有数据后，一直卡住，程序不退出
    // 使用take(3) 可以读取3个后，不再阻塞
    // let messages = subscriber.into_stream();
    // let messages = subscriber.into_stream().take(3);

    // 过滤，只保留数字类型的值
    // 此种方式，可能无法取到第三个值，一直卡住
    // 可以 client.publish 多几个数字
    let messages = subscriber.into_stream().filter(|item| match item {
        Ok(msg) if msg.content.len() == 1 => true,
        _ => false,
    }).map(|msg| msg.unwrap().content).take(3);

    tokio::pin!(messages);

    while let Some(msg) = messages.next().await {
        println!("got = {:?}", msg);
    }

    Ok(())
}
