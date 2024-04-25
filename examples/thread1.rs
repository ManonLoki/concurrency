use std::sync::mpsc::channel;

use anyhow::Result;
use rand::random;

/// 最大的消费者数量
const MAX_PRODUCER: usize = 4;

#[derive(Debug)]
#[allow(dead_code)]
struct Message {
    id: usize,
    data: u64,
}
impl Message {
    fn new(id: usize, data: u64) -> Self {
        Self { id, data }
    }
}

fn main() -> Result<()> {
    let (tx, rx) = channel();

    // 产生消息
    for i in 0..MAX_PRODUCER {
        let tx = tx.clone();
        std::thread::spawn(move || {
            loop {
                // 0-254 *10
                let random_data = random::<u8>() as u64;
                let message = Message::new(i, random::<u64>());
                tx.send(message).unwrap();

                // 随机退出条件
                if random_data % 5 == 0 {
                    break;
                }

                std::thread::sleep(std::time::Duration::from_millis(random_data * 10));
            }

            println!("producer {} exit", i);
        });
    }

    // 释放掉最外层多余的tx
    drop(tx);

    let consumer = std::thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(message) => {
                    println!(
                        "consumer {:?} receive message {:?}",
                        std::thread::current().id(),
                        message
                    );
                }
                Err(err) => {
                    println!(
                        "consumer {:?} exit with error {:?}",
                        std::thread::current().id(),
                        err
                    );
                    break;
                }
            }
        }

        println!("consumer   exit");
    });

    consumer
        .join()
        .map_err(|err| anyhow::anyhow!(format!("consumer exit with error {:?}", err)))?;

    println!("main thread exit");

    Ok(())
}
