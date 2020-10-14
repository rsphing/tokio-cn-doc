use bytes::Bytes;
use tokio::sync::mpsc;
use std::option::Option::Some;
use mini_redis::client;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        val: Bytes,
    },
}


#[tokio::main]
async fn main() {
    let (mut tx, mut rx) = mpsc::channel(32);
    // clone 发送者完成多个任务的发送
    let mut tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        let cmd = Command::Get {
            key: "hello".to_string(),
        };

        tx.send(cmd).await.unwrap();
    });

    let t2 = tokio::spawn(async move {
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
        };
        tx2.send(cmd).await.unwrap();
    });

    // while let Some(message) = rx.recv().await {
    //     println!("Got = {}", message);
    // }

    let manager = tokio::spawn(async move {
        //建立一个与服务器的链接
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // 开始接收消息
        while let Some(message) = rx.recv().await {
            use Command::*;
            match message {
                Get { key } => {
                    client.get(&key).await.unwrap();
                }
                Set { key, val } => {
                    client.set(&key, value).await.unwrap();
                }
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}