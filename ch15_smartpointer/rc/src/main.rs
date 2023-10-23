use std::rc::Rc;

fn main() {
    //Box
    {
        use crate::ListBox::{Cons, Nil};
        let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
        let b = Cons(3, Box::new(a));
        //let c = Cons(4, Box::new(a));//Error,a的所有权已转移
    }
    //Rc，引用计数，允许多所有权，只读(只能用于单线程场景)
    {
        use crate::ListRc::{Cons, Nil};
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        //Rc::clone()，克隆a所包含的Rc<ListRc>，引用计数从1增加到2，a和b共享Rc<ListRc>中数据的所有权
        let b = Cons(3, Rc::clone(&a));
        //调用a.clone()同理，但是并非深拷贝，而是只增加引用计数
        let c = Cons(4, a.clone());
    }
    //Rc::strong_count函数获取引用计数
    //当离开作用域时计数减1，实质时通过Drop的trait实现，当计数为0时则清理
    {
        use crate::ListRc::{Cons, Nil};
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("Count after creating a = {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("Count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
}

enum ListBox {
    Cons(i32, Box<ListBox>),
    Nil,
}

enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}
