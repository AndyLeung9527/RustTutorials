use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    {
        //创建一个信道，mpsc是多个生产者，单个消费者(multiple producer,single consumer)的缩写
        //其返回一个元组，第一个元素是发送者tx，第二个元素是接收者rx
        let (tx, rx) = mpsc::channel();
        //创建新线程负责发送
        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });
        //主线程接收，recv方法会阻塞等待，try_recv方法不会阻塞而是立即获取
        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

    //通过信道发送多个值
    {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
        //将rx作为迭代器，当信息被关闭时结束
        for received in rx {
            println!("Got: {}", received);
        }
    }

    //通过克隆发送者来创建多个生产者
    {
        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
        for received in rx {
            println!("Got: {}", received);
        }
    }
}
