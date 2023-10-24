use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    //Mutex<T>互斥器
    {
        let m = Mutex::new(5);
        {
            let mut num = m.lock().unwrap(); //获取锁，属于阻塞调用；Mutex<T>提供了内部可变性，类似RefCell<T>，因此可以获取内部可变的数据
            *num = 6;
            //离开作用域时自动释放锁
        }
        println!("m = {:?}", m);
    }
    //线程间共享Mutex<T>
    //因为move会转移所有权，不能达到锁多线程引用，因此需要使用多所有权Rc<T>；然而Rc<T>只允许用于单线程，因此需要使用Arc<T>代替(性能比Rc<T>低)，它们提供的API一致
    {
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
        println!("Result: {}", *counter.lock().unwrap());
    }
}

//std::marker::Send trait
//实现了Send的类型值的所有权可以在线程间传送，完全由Send的类型组成的类型也会自动被标记为Send

//std::marker::Sync trait
//实现了Sync的类型可以安全的在多个线程中拥有其值的引用，对于任意类型T，如果&T(T的不可变引用)是Send的话T就是Sync的，完全由Sync的类型组成的类型也是Sync
