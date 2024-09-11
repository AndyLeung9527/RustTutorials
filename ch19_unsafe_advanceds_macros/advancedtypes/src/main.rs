use std::collections::HashMap;

fn main() {
    //为了类型安全和抽象而使用 newtype 模式
    {
        //使用Millimeters和Meters结构体封装u32的值来表示单位，避免混用了u32值
        struct Millimeters(u32);
        struct Meters(u32);

        //隐藏内部的泛型类型
        struct People(HashMap<i32, String>);
    }

    //类型别名
    {
        type Kilometers = i32; //类型别名，使用type关键字赋予现有类型的另一个命名，会被完全当作i32类型
        let x: i32 = 5;
        let y: Kilometers = 5;
        println!("x + y = {}", x + y);

        //使用类型别名减少冗长的类型被重复编写
        type Thunk = Box<dyn Fn() + Send + 'static>;
        let f: Thunk = Box::new(|| println!("hi"));
        fn takes_long_type(f: Thunk) {} //fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {}
    }

    //从不返回的never type
    {
        //特殊类型!，在函数从不返回的时候充当返回值，函数被称为发散函数
        //使用此类型时，函数内部必须发生某种终止程序运行的情况，例如panic或者调用std::process::exit函数
        fn bar() -> ! {
            panic!();
            std::process::exit(0);
        }
    }

    //动态大小类型(称作DST或unsized types)和Sized trait
    {
        //let s1: str = "Hello there!";//Error，str是一个DST，只有在运行时才知道大小，不能直接创建这个变量，一般与所有类型的指针结合使用，比如Box<str>、Rc<str>
        let s1: &str = "Hello there!"; //Correct，&str不是DST，储存了地址和长度，编译时即可确定大小

        //Sized trait决定一个类型的大小是否在编译时可知，泛型参数默认已增加
        fn generic<T>(t: T) {} //实际上是fn generic<T: Sized>(t:T){}

        //使用如下语法解除Sized trait的限制
        fn generic_unsized<T: ?Sized>(t: &T) {} //T须要变为&T，因为其类型可能不是Sized，须要使用包含地址和长度的&指针
    }
}
