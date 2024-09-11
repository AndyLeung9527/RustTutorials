use std::slice;

fn main() {
    //裸指针
    {
        let mut num = 5;
        let r1 = &num as *const i32; //不可变裸指针
        let r2 = &mut num as *mut i32; //可变裸指针

        //创建裸指针无须unsafe，但是访问须要
        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }

    //调用不安全函数或方法
    {
        unsafe fn dangerous() {} //通过unsafe声明不安全函数
        unsafe {
            dangerous();
        }
    }

    //创建不安全代码的安全抽象
    {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let (_a, _b) = split_at_mut(&mut v[..], 3); //无须unsafe，不安全代码的安全抽象
        fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = values.len();
            let ptr = values.as_mut_ptr(); //获取slice的裸指针，无须unsafe
            assert!(mid <= len);
            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
            }
        }
    }

    //extern函数调用外部代码须加unsafe
    {
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

    //可变静态变量的访问和修改
    {
        static mut COUNTER: u32 = 0;
        unsafe {
            COUNTER += 1;
        }
        unsafe {
            println!("COUNTER: {COUNTER}");
        }
    }

    //实现不安全trait，trait的实现也必须标记为unsafe
    {
        unsafe trait Foo {}
        unsafe impl Foo for i32 {}
    }
}

//集成C标准库中的abs函数
extern "C" {
    fn abs(input: i32) -> i32;
}

//使用extern创建一个C语言能调用的Rust函数
//#[no_mangle]禁用Rust编译器对函数名称的mangle
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
