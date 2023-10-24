use std::thread;
use std::time::Duration;

fn main() {
    let demo_index = 1;
    if demo_index == 1 || demo_index == 2 {
        //创建一个线程
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        //若主线程结束，其子线程也会被强制结束，因此上述子线程最多只能打印到5
        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        if demo_index == 2 {
            //调用join方法会等待线程结束，来确保子线程在main退出前执行完毕
            handle.join().unwrap();
        }
    } else if demo_index == 3 {
        let v = vec![1, 2, 3];
        // let handle = thread::spawn(|| {
        //     println!("Here's a vector: {:?}", v);
        // });//Error，线程中无法知道v的引用是否一直有效
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        }); //使用move获取v的所有权可解决
        handle.join().unwrap();
    }
}
