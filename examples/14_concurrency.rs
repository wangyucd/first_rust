// ============================================================
// Rust 基础教程 14: 并发编程 (Concurrency)
// 运行: cargo run --example 14_concurrency
// ============================================================

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

fn main() {
    // ==================== 创建线程 ====================
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("子线程: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..=3 {
        println!("主线程: {}", i);
        thread::sleep(Duration::from_millis(150));
    }

    // join 等待子线程完成
    handle.join().unwrap();

    // ==================== move 闭包 ====================
    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("子线程拥有数据: {:?}", data);
    });
    handle.join().unwrap();
    // println!("{:?}", data); // ❌ data 已移动到子线程

    // ==================== 消息传递 (mpsc) ====================
    // mpsc = multiple producer, single consumer

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec!["你好", "来自", "子线程"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // 接收消息
    for received in rx {
        println!("收到: {}", received);
    }

    // ==================== 多个生产者 ====================
    let (tx, rx) = mpsc::channel();

    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let msg = format!("线程 {} 的消息", i);
            tx_clone.send(msg).unwrap();
        });
    }

    drop(tx); // 关闭原始发送端

    for received in rx {
        println!("收到: {}", received);
    }

    // ==================== 共享状态: Mutex ====================
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终计数: {}", *counter.lock().unwrap());

    // ==================== Arc<Mutex<T>> 模式 ====================
    // Arc: 原子引用计数，线程安全的 Rc
    // Mutex: 互斥锁，保证同一时间只有一个线程访问数据

    let shared_vec = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for i in 0..5 {
        let shared = Arc::clone(&shared_vec);
        let handle = thread::spawn(move || {
            let mut vec = shared.lock().unwrap();
            vec.push(i);
            println!("线程 {} 添加了 {}", i, i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("共享向量: {:?}", shared_vec.lock().unwrap());

    // ==================== 实用示例: 并行计算 ====================
    let data: Vec<i64> = (1..=1000).collect();
    let chunk_size = 250;
    let mut handles = vec![];

    for chunk in data.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let handle = thread::spawn(move || -> i64 { chunk.iter().sum() });
        handles.push(handle);
    }

    let total: i64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("并行求和 (1..1000): {}", total);

    // ==================== 实用示例: 生产者-消费者 ====================
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    // 生产者
    let producer = thread::spawn(move || {
        for i in 1..=10 {
            tx.send(i * i).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
        println!("生产者完成");
    });

    // 消费者
    let mut consumers = vec![];
    for id in 0..3 {
        let rx = Arc::clone(&rx);
        let consumer = thread::spawn(move || {
            loop {
                let result = {
                    let rx = rx.lock().unwrap();
                    rx.recv()
                };
                match result {
                    Ok(value) => println!("消费者 {} 处理: {}", id, value),
                    Err(_) => break,
                }
            }
            println!("消费者 {} 完成", id);
        });
        consumers.push(consumer);
    }

    producer.join().unwrap();
    for consumer in consumers {
        consumer.join().unwrap();
    }

    println!("\n✅ 并发编程教程完成！");
}