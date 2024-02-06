use rand::{random, Rng, RngCore};
use std::thread;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::select;
use tokio::sync::oneshot;

/// 同时等待多个异步操作的结果，并且对其结果进行进一步处理
/// select! 最多可以支持 64 个分支，每个分支形式如下
///  <模式> = <async 表达式> => <结果处理>,
#[tokio::main]
async fn main() {
    println!("# select!");
    println!("## tokio::select!");
    let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        // tx1.send("one");
        // 等待 some_operation操作完成
        // 或者处理`oneshot`的关闭通知
        tokio::select! {
            val = some_operation() => {
                tx1.send("one");
            }
            val = tx1.closed() => {
                println!("tx1 was closed: {:?}", val);
            }
        }
    });
    tokio::spawn(async {
        tx2.send("two");
    });

    // 任何一个 select 分支完成后，都会继续执行后面的代码，没被执行的分支会被丢弃( dropped )
    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        },
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }
    println!("select is over");
    println!("\n### 在分支中进行 TCP 连接");

    let (tx, rt) = oneshot::channel();

    // 启用线程
    tokio::spawn(async {
        // 线程sleep 3s
        thread::sleep(Duration::from_secs(3));
        // 发送完成信息
        match tx.send("down") {
            Ok(_) => {println!("发送完成")}
            Err(_) => {println!("发送异常")}
        };
    });

    // 匹配
    tokio::select! {
        listener = TcpStream::connect("127.0.0.1:8080") => {
            println!("已连接上127.0.0.1:8080");
        }
        val = rt => {
            println!("received message first {:?}", val);
        }
    };
    println!("select is over!");

    println!("\n## 返回值");
    tokio::select! {
        val = computation1() => {
            println!("select computation1： {}", val);
        }
        val = computation2() => {
            println!("select computation2： {}", val);
        }
    };

    // 模式匹配
    // <模式> = <async 表达式> => <结果处理>,
    println!("\n## 模式匹配");
    let (mut tx, mut rx) = tokio::sync::mpsc::channel(128);

    let mut done = false;
    let operation = action(None);
    tokio::pin!(operation);

    tokio::spawn(async move {
        let _ = tx.send(1).await;
        let _ = tx.send(3).await;
        let _ = tx.send(2).await;
    });

    loop {
        tokio::select! {
            res = &mut operation, if !done => {
                done = true;
                println!("done set true");
                if let Some(v) = res {
                    println!("GOT = {}", v);
                    return;
                }
            }
            Some(v) = rx.recv() => {
                println!("recv {v}");
                if v % 2 == 0 {
                    // `.set` 是 `Pin` 上定义的方法
                    operation.set(action(Some(v)));
                    done = false;
                }
            }
        }
    }
}

async fn action(input: Option<i32>) -> Option<String>{
    // 若 input（输入）是None，则返回 None
    // 事实上也可以这么写: `let i = input?;`
    let i = match input {
        Some(input) => input,
        None => return None,
    };

    Some("hello".to_string())
}

async fn computation1() -> String {
    let sec = rand::thread_rng().gen_range(1..=2);
    println!("computation1 sleep {}s", sec);
    thread::sleep(Duration::from_secs(sec));
    "1".to_string()
}

async fn computation2() -> String {
    let sec = rand::thread_rng().gen_range(1..=2);
    println!("computation2 sleep {}s", sec);
    thread::sleep(Duration::from_secs(sec));
    "2".to_string()
}

/// 异步任务
async fn some_operation() {
    println!("some_operation start");
    // 模拟耗时操作
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    println!("some_operation is completed");
}
