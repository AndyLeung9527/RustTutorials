use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    //循环引用依赖会导致内存泄漏
    {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(5, RefCell::new(Rc::clone(&a)))); //b->a
        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b); //修改a->b，形成循环依赖(b->a->b->a...)
        }
        println!("b rc count after changing b = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));
        //println!("a next item = {:?}", a.tail());//Error，将会循环至栈溢出

        //作用域结束后，b会丢弃，引用计数从2减为1，因为引用计数仍不是0，未能回收；a同理；因为循环引用依赖导致内存泄漏
    }

    //使用弱引用避免循环引用依赖
    //调用Rc::downgrade时会得到Weak<T>类型的智能指针，weak_count加1，记录存在多少个Weak<T>引用(计数无需为0才能丢弃)
    //调用Weak<T>实例的upgrade方法，返回Option<Rc<T>>，若Rc<T>被丢弃则结果时None，否则是Some
    {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch)
            );
            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf)
            );

            //作用域结束后，branch被丢弃，指向的Rc<T>也被丢弃(leaf对branch的Rc<T>是弱依赖，引用计数不被累加)，leaf的引用计数减1
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        //作用域结束后，leaf被丢弃，引用计数减1，即为0，不会造成循环引用依赖和内存泄漏
    }
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
//若使用Rc<T>，某Node的chidren的parent指向自己，形成循环依赖引用
//父节点应该拥有其子节点，当父节点丢弃时子节点也被丢弃；然而子节点丢弃，其父节点不应该丢弃；因此需要使用weak弱引用，依赖父节点时强引用数不会被累加
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
