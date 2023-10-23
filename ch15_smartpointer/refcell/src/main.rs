use crate::ListRcRefCell::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    //RefCell
    {
        let m = Messenger {
            messages: vec![],
            message_refcell: RefCell::new(vec![]),
        };
        //m.messages.push(String::from("message"));//Error，self为不可变引用，编译器建议使用&mut self替代
        m.message_refcell.borrow_mut().push(String::from("message")); //调用RefCell的borrow_mut方法获取可变引用
        let n = m.message_refcell.borrow(); //调用RefCell的borrow方法获取不可变引用
    }

    //Rc结合RefCell
    //Rc允许多个所有者，不过只提供不可变访问，结合Rc与RefCell就可以实现可变访问
    {
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
        *value.borrow_mut() += 10;
        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
}

struct Messenger {
    messages: Vec<String>,
    message_refcell: RefCell<Vec<String>>,
}

#[derive(Debug)]
enum ListRcRefCell {
    Cons(Rc<RefCell<i32>>, Rc<ListRcRefCell>),
    Nil,
}
