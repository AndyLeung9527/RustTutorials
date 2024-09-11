fn main() {
    //函数指针
    {
        //fn函数指针向函数传递闭包，即使用函数作为另一个函数的参数
        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            f(arg) + f(arg)
        }
        fn add_one(x: i32) -> i32 {
            x + 1
        }
        let answer = do_twice(add_one, 5); //使用函数
        println!("The answer is: {answer}");

        let answer2 = do_twice(|i| i + 1, 5); //使用内联定义
        println!("The answer2 is: {answer2}");

        enum Status {
            Value(u32),
            Stop,
        }
        let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect(); //使用构造函数
        println!("Len: {}", list_of_statuses.len());
    }

    //返回闭包
    {
        /* Error，错误指向储存闭包Sized trait，无法知道储存闭包所需要的空间
        fn returns_closure() -> dyn Fn(i32) -> i32 {
            |x| x + 1
        }*/
        //使用Box<T>指针封装，大小是固定的
        fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }
    }
}
