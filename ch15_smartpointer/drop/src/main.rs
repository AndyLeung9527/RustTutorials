fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomSmartPointers created.");
    //c.drop();//Error，不允许显式调用，可使用std::mem::drop代替，如下
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    //类型实例离开作用域时自动执行，无需显式调用，相当于析构函数
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
