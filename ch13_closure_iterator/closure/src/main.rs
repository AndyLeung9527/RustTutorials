use std::thread;
use std::time::Duration;

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    //定义闭包并保存在变量中
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    /*相同行为的多个闭包语法
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }//函数定义
    let add_one_v2 = |x: u32| -> u32 { x + 1 };//完整标注的闭包定义
    let add_one_v3 = |x|             { x + 1 };//闭包定义中省略了类型注解
    let add_one_v4 = |x|               x + 1  ;//闭包体只有一个表达式，可去掉大括号
     */

    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    //let n = example_closure(5);//Error，不符合编译器为闭包定义的参数和返回值中推断出的具体类型

    //闭包借用不可变引用
    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);
        let only_borrows = || println!("From closure: {:?}", list);
        println!("Before calling closure: {:?}", list);
        only_borrows();
        println!("After calling closure: {:?}", list);
    }

    //闭包借用可变引用
    {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);
        let mut borrows_mutably = || list.push(7);
        //println!("Before calling closure: {:?}", list);//Error，可变借用未结束时不允许有其它的借用
        borrows_mutably();
        println!("After calling closure: {:?}", list);
    }

    //闭包获取所有权，可以使用move关键字
    {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);
        thread::spawn(move || println!("From thread: {:?}", list))
            .join()
            .unwrap();
        //println!("After calling closure: {:?}", list);//Error，list所有权已被移动到闭包中
    }

    /*闭包trait
    FnOnce(所有闭包都至少实现)：只能调用一次，参数会被移出闭包，fn call_once(self, [Args]) -> [Return];
    FnMut：可被调用多次，参数不会被移出闭包，可被修改，fn call_mut(&mut self, [Args]) -> [Return];
    Fn：可被调用多次，参数既不移出闭包，也不会被修改，fn call(&self, [Args]) -> [Return];
     */

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    let mut sort_operations: Vec<String> = Vec::new();
    let value = String::from("by key called");
    list.sort_by_key(|r| {
        //sort_operations.push(value);//Error，FnMut可调用多次，push只能执行一次(value所有权已转移)
        r.width
    });

    println!("{:#?}", list);
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked()) //闭包
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if (num_red > num_blue) {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
