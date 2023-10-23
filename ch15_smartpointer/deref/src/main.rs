use std::ops::Deref;

fn main() {
    //A
    //追踪指针的值
    {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        //assert_eq!(5, y);//Error，类型不同，不允许比较数字和数字的引用
        assert_eq!(5, *y); //*号解引用之后可以比较
    }
    //B
    //使用Box<T>实现同样效果
    {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    //B与A不同在于将y设置为一个指向x值拷贝的Box<T>实例，而非指向x值的引用

    //使用自定义智能指针
    {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    //没有实现Deref的trait的话，编译器只能解引用&引用类型

    //Deref的强制转换
    //如果类型T实现了Deref<Target = U>的trait，并且y是T的一个变量时：
    //1.当使用*x时，等同于*Deref::deref(&x)，即先调用deref方法，再进行普通的解引用操作(T既不能是引用，也不能是裸指针)
    //2.类型&T的值可以被强制转换为&U的值
    //3.类型T隐含地实现了类型U的所有不可变的方法
    //综上所述：Deref的强制转换，其实就是在函数或方法传参时，实现了Deref的类型的引用可以转换为通过Deref所能转换的类型的引用
    //因此例子中：&MyBox<String>==&String==&str(标准库中提供了String的Deref实现)
    {
        //I.使用deref强制转换
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
        //II.显式使用deref，等同于：
        let m = MyBox::new(String::from("Rust"));
        let n: &String = Deref::deref(&m);
        let s: &str = Deref::deref(n);
        hello(s);
        //III.若没有deref，等同于：
        let m = MyBox::new(String::from("Rust"));
        hello(&(*m)[..]);
    }

    //Deref重载不可变引用的*运算符
    //DerefMut重载可变引用的*运算符(T:DerefMut<Target=U>转换从&mut T到&mut U，若T:Deref<Target=U>转换从&mut T到&U；Rust能将可变引用转为不可变引用，反之不行)
    //DerefMut同样适用强制转换规则
}

//自定义智能指针
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
//继承Deref实现解引用
impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0 //返回*运算符访问的值的引用，.0访问元组结构体的第一个元素
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
