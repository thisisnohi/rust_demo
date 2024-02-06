use std::future::Future;
use std::ops::Add;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};

use crossbeam::channel;
use futures::future::poll_fn;
use futures::task;
use futures::task::{ArcWake, SpawnExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    println!("# 深入 Tokio 背后的异步原理");

    println!("\nFuture");

    let stream = my_async_fn();

    println!("before stream.await");
    // 等待异步操作完成
    stream.await;

    println!("after stream.await");

    // 1. 等待某个特定时间点的到来
    // 2. 在标准输出打印文本
    // 3. 生成一个字符串
    println!("\n实现 Future");

    let delay = Delay {
        when: Instant::now().add(Duration::from_secs(2)),
    };
    println!("Delay定义完成:{:?}", Instant::now());
    let rs = delay.await;
    println!("rs: {} now:{:?}", rs, Instant::now());

    println!("\n## 执行器( Excecutor )");
    println!("\n### mini tokio");
    let mut mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        let when = Instant::now().add(Duration::from_secs(2));
        let future = Delay { when };
        println!("定义Delay完成");
        let out = future.await;
        println!("out: {}", out);
    });
    println!("调用异常任务后");
    mini_tokio.run();
    println!("任务执行完成");

    println!("\n## Waker");
    println!("\n### 处理 wake 通知");

    println!("\n## 一些遗留问题");
    println!("\n### 在异步函数中生成异步任务");
    let when = Instant::now().add(Duration::from_millis(10));
    let mut delay = Some(Delay { when });

    poll_fn(move |cx| {
        let mut delay = delay.take().unwrap();
        let res = Pin::new(&mut delay).poll(cx);
        assert!(res.is_pending());
        tokio::spawn(async move {
            delay.await;
        });
        Poll::Ready(())
    })
    .await;
}

//// 第一版
// type Task = Pin<Box<dyn Future<Output = ()> + Send>>;
// struct MiniTokio {
//     tasks: VecDeque<Task>,
// }
//
// impl MiniTokio {
//     fn new() -> MiniTokio {
//         MiniTokio {
//             tasks: VecDeque::new(),
//         }
//     }
//
//     // 生成一个Future并放入mini-tokio实例的任务队列中
//     fn spawn<F>(&mut self, future: F)
//     where
//         F: Future<Output = ()> + Send + 'static,
//     {
//         self.tasks.push_back(Box::pin(future))
//     }
//
//     fn run(&mut self) {
//         let waker = task::noop_waker();
//         let mut cx = Context::from_waker(&waker);
//
//         while let Some(mut task) = self.tasks.pop_front() {
//             if task.as_mut().poll(&mut cx).is_pending() {
//                 self.tasks.push_back(task);
//             }
//         }
//     }
// }

/// 第二版
struct MiniTokio {
    scheduled: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>,
}

struct Task {
    // `Mutex`是为了让`Task`实现`Sync`特征，它能保证同一时间只有一个线程可以访问`Future`
    // 事实上`Mutex`并没有在Tokio中被使用，这里我们只是为了简化：Tokio的真实代码实在太长了 :D
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor: channel::Sender<Arc<Task>>,
}

impl Task {
    fn schedule(self: &Arc<Self>) {
        let _ = self.executor.send(self.clone());
    }

    fn poll(self: Arc<Self>) {
        // 基于 Task 实例创建一个 waker, 它使用了之前的 `ArcWake`
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);

        // 没有其他线程在竞争锁时，我们将获取到目标future
        let mut future = self.future.try_lock().unwrap();

        // 对future进程poll
        let _ = future.as_mut().poll(&mut cx);
    }

    // 使用给定的 future 来生成新的任务
    //
    // 新的任务会被推到 `sender` 中，接着该消息通道的接收端就可以获取该任务，然后执行
    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });

        let _ = sender.send(task);
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}

impl MiniTokio {
    /// 初始化一个新的mini-tokio实例
    fn new() -> MiniTokio {
        let (sender, scheduled) = channel::unbounded();
        MiniTokio { sender, scheduled }
    }

    // 生成一个Future并放入mini-tokio实例的任务队列中
    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(future, &self.sender);
    }

    // 从消息通道中接收任务，然后通过poll来执行
    fn run(&self) {
        while let Ok(mut task) = self.scheduled.recv() {
            task.poll();
        }
    }
}

struct Delay {
    when: Instant,
}

// 为Delay 实现Future
impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            // 时间到了，Future 可以结束
            println!("Hello world");
            // Future执行结束，返回"donw"字符串
            Poll::Ready("done")
        } else {
            // 目前忽略下面这行代码
            // cx.waker().wake_by_ref();
            // Poll::Pending

            // 使用Waker
            // 为当前任务克隆一个Waker的句柄
            let waker = cx.waker().clone();
            let when = self.when;

            // 生成一个计时器任务
            thread::spawn(move || {
                let now = Instant::now();
                if when > now {
                    thread::sleep(now - when);
                }
                waker.wake();
            });

            Poll::Pending
        }
    }
}

async fn my_async_fn() {
    let stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    println!("Connected to server");
}
