use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main() {
    println!("消息传递");

    // 创建新通道，缓冲队列长度是32
    // let (tx, mut rx) = mpsc::channel(32);
    // let tx2 = tx.clone();
    //
    // // 发送消息
    // tokio::spawn(async move {
    //     tx.send("sending from first handle").await;
    // });
    // tokio::spawn(async move {
    //     tx2.send("sending from second handle").await;
    // });
    //
    // while let Some(message) = rx.recv().await {
    //     println!("Got: {}", message);
    // }

    let (tx, mut rx) = mpsc::channel(32);
    // 拷贝一个发送者
    let tx2 = tx.clone();

    // 将消息通道接收者rx的所有权移到管理任务中
    let manager = tokio::spawn(async move {
        // 建立一个redis连接
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // 开始接收消息
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Set { key, val, resp } => {
                    let rs = client.set(&key, val).await;
                    // 响应
                    // 忽略错误
                    let _ = resp.send(rs);
                }
                Command::Get { key, resp } => {
                    let rs = client.get(&key).await;
                    // match rs {
                    //     Ok(val) => {
                    //         println!("GET {} is {:?}", key, val);
                    //         // 响应
                    //         // 忽略错误
                    //         let _ = resp.send(Result::Ok(val));
                    //     }
                    //     Err(msg) => {
                    //         println!("GET {} Error {}", key, msg);
                    //         // 响应
                    //         // 忽略错误
                    //         let _ = resp
                    //             .send(Result::Err(format!("没有对应key[{}]的数据", key).into()));
                    //     }
                    // }
                    // 可以直接返回，不需要上述代码判断
                    // 响应
                    // 忽略错误
                    let _ = resp.send(rs);
                }
            }
        }
    });

    // 生成两个任务，一个用于设置key值，一个用于获取key值
    let t1 = tokio::spawn(async move {
        let (req_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "hello".to_string(),
            resp: req_tx,
        };
        // 发送请求
        tx.send(cmd).await.unwrap();
        // 等待回复
        let rs = resp_rx.await;
        println!("Got response: {:?}", rs)
    });

    let t2 = tokio::spawn(async move {
        let (req_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "hello".to_string(),
            val: "bar".into(),
            resp: req_tx,
        };
        // 发送请求
        tx2.send(cmd).await.unwrap();
        // 等待回复
        let rs = resp_rx.await;
        println!("Got response: {:?}", rs)
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
/// 管理任务可以使用该发送端将命令执行的结果传回给发出命令的任务
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;
#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}
