use crate::List::{Cons, Nil};

fn main() {
    //Box<T>在堆上储存数据
    {
        let b = Box::new(5);
        println!("b = {}", b);
    }
    //Box创建递归类型
    {
        // let list = Cons(1, Cons(2, Cons(3, Nil)));
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }
}

enum List {
    //Cons(i32, List),//Error，递归时无法计算需要分配多少空间
    Cons(i32, Box<List>), //Box<T>是一个指针，大小是固定的(usize)，指向另一个位于堆上的List值
    Nil,
}
