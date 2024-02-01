use bytes::{Buf, Bytes, BytesMut};
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{Arc, Mutex};

use mini_redis::frame::Error;
use mini_redis::{Command, Frame, Result};
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
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

pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    fn new(stream: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(stream),
            // 分配一个缓冲区，具有4kb的缓冲长度
            buffer: BytesMut::with_capacity(4096),
        }
    }

    /// 从连接读取一个帧
    ///
    /// 如果遇到EOF，则返回 None
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            // 1. 读取一个frame
            // 尝试从缓冲区的数据中解析出一个数据帧，
            // 只有当数据足够被解析时，才返回对应的帧
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            // 2. 如果不是一个完整的fream,继续读取
            // 如果缓冲区中的数据还不足以被解析为一个数据帧，
            // 那么我们需要从 socket 中读取更多的数据
            //
            // 读取成功时，会返回读取到的字节数，0 代表着读到了数据流的末尾
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                // 代码能执行到这里，说明了对端关闭了连接，
                // 需要看看缓冲区是否还有数据，若没有数据，说明所有数据成功被处理，
                // 若还有数据，说明对端在发送帧的过程中断开了连接，导致只发送了部分数据
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    async fn write_decimal(&mut self, val: u64) -> std::io::Result<()> {
        use std::io::Write;

        // Convert the value to a string
        let mut buf = [0u8; 20];
        let mut buf = Cursor::new(&mut buf[..]);
        write!(&mut buf, "{}", val)?;

        let pos = buf.position() as usize;
        self.stream.write_all(&buf.get_ref()[..pos]).await?;
        self.stream.write_all(b"\r\n").await?;

        Ok(())
    }
    /// 从连接读取一个帧
    ///
    /// 如果遇到EOF，则返回 None
    pub async fn writer_frame<'a>(&mut self, frame: &Frame) -> io::Result<()> {
        match frame {
            Frame::Simple(val) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await;
            }
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await;
            }
            Frame::Integer(val) => {
                self.stream.write_u8(b':').await?;
                self.write_decimal(*val).await?;
            }
            Frame::Bulk(val) => {
                let len = val.len();
                self.stream.write_u8(b'$').await;
                self.write_decimal(len as u64).await?;
                self.stream.write_all(val).await?;
                self.stream.write_all(b"\r\n").await;

            }
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await;
            }
            Frame::Array(val) => unimplemented!()
        }
        self.stream.flush().await;
        // 返回
        Ok(())
    }
    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        // 创建 `T: Buf`类型
        let mut buf = Cursor::new(&self.buffer[..]);

        // 检查是否读取了足够解析出一个帧的数据
        match Frame::check(&mut buf) {
            Ok(_) => {
                // 获取组成该帧的字节数
                let len = buf.position() as usize;
                // 在解析之前，重置内部的坐标位置
                buf.set_position(0);
                // 解析帧
                let frame = Frame::parse(&mut buf)?;
                // 解析完成，将缓冲区该帧的数据移除
                self.buffer.advance(len);
                // 解析完成
                Ok(Some(frame))
            }
            Err(Error::Incomplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
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

        connection.writer_frame(&response).await.unwrap();
    }
}
