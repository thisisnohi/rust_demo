use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use bytes::Bytes;
use mini_redis::{Command, Error, Frame, Result};
use tokio::io;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    // tokio::spawn(async {
    //     // future is not `Send` as this value is used across an await
    //     let rc = Rc::new(5);
    //     let rc = Arc::new(Mutex::new(5));
    //     process2().await;
    //     println!("{:?}", rc);
    // });

    println!("Redis 服务端");
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let mark_twain = "Samuel Clemens";

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (stream, addres) = listener.accept().await.unwrap();
        println!("客户端[{}]已连接", addres);
        let db_clone = db.clone();
        // 处理链接请求
        // 为每一个连接生成一个新的任务
        // 'stream' 移交至新任务中
        tokio::spawn(async move {
            process(stream, db_clone).await;
        });
    }
}

struct Connection {
    stream: TcpStream,
    buffer: Vec<u8>,
    cursor: usize,
}

impl Connection {
    fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            // 分配一个缓冲区，具有4kb的缓冲长度
            buffer: vec![0; 4096],
            cursor: 0,
        }
    }

    /// 从连接读取一个帧
    ///
    /// 如果遇到EOF，则返回 None
    pub async fn read_frame<'a>(&mut self) -> Result<Option<Frame>> {
        loop {
            // 1. 读取一个frame
            // 尝试从缓冲区的数据中解析出一个数据帧，
            // 只有当数据足够被解析时，才返回对应的帧
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            // 确保缓冲区长度足够，若不够，需要增加长度
            if self.buffer.len() == self.cursor {
                self.buffer.resize(self.cursor * 2, 0);
            }

            // 从游标位置开始将数据读入缓冲区
            let n = self.stream.read(&mut self.buffer[self.cursor..]).await?;

            if 0 == n {
                if self.cursor == 0 {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            } else {
                self.cursor += n;
            }
        }
    }

    /// 从连接读取一个帧
    ///
    /// 如果遇到EOF，则返回 None
    pub async fn writer_frame<'a>(&mut self, frame: &Frame) -> io::Result<()> {
        todo!()
    }

    fn parse_frame(&self) -> Result<Option<Frame>> {
        todo!()
    }
}

async fn process(stream: TcpStream, db: Db) {
    // `Connection` 对于 redis 的读写进行了抽象封装，因此我们读到的是一个一个数据帧frame(数据帧 = redis命令 + 数据)，而不是字节流
    // `Connection` 是在 mini-redis 中定义
    let mut connection = Connection::new(stream);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                // 获取锁
                let mut map = db.lock().unwrap();
                map.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Command::Get(cmd) => {
                // 获取锁
                let mut map = db.lock().unwrap();
                if let Some(val) = map.get(cmd.key()) {
                    Frame::Bulk(val.clone().into())
                } else {
                    println!("无此键[{}]", cmd.key());
                    Frame::Error("unimplemented".to_string())
                    // Frame::Null
                }
            }
            _ => Frame::Error("unimplemented".to_string()),
        };

        connection.write_frame(&response).await.unwrap();
    }
}
