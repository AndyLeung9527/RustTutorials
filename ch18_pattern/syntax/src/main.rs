fn main() {
    //匹配字面值
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything"),
    };

    //匹配命名变量
    let x = Some(5);
    let y = 10;
    match x {
        //不匹配x中定义的值，跳过
        Some(50) => println!("Got 50"),
        //会开始一个新的作用域，引入新变量y，而非之前声明的y，新变量绑定匹配到Some中的值，即x中Some内部的值，执行得y = 5
        Some(y) => print!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }
    //执行完毕后，match作用域结束，内部y回收，回到之前声明的y，得y = 10
    println!("at the end: x = {:?}, y = {y}", x);

    //多个模式
    let x = 1;
    match x {
        //或
        1 | 2 => println!("one or two"),
        //闭区间范围，只能用于数字或char
        1..=5 => println!("one through five"),
        _ => println!("anything"),
    }

    //解构结构体
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    let Point { x, y } = p; //变量名一致时可简写
    println!("a:{},b:{},x:{},y:{}", a, b, x, y);

    //解构并匹配模式
    let p = Point { x: 0, y: 7 };
    match p {
        //指定字段y匹配字面值0，x为任意值
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        //指定字段x匹配字面值0，y为任意值
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }

    //解构枚举
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
        _ => {
            println!("Other");
        }
    }

    //匹配嵌套的枚举
    let msg = Message::MutiColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::MutiColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::MutiColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }

    //解构结构体和元组
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    //使用_忽略值
    foo(3, 4);

    //使用_匹配成员
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    //使用_不会获取所有权
    let s = Some(String::from("Hello"));
    /*错误，获取了所有权
    if let Some(s_copy) = s {
        println!("found a string")
    };*/
    if let Some(_) = s {
        println!("found a string")
    };
    println!("{:?}", s);

    //使用..忽略剩余值，会扩展为所需要的值得数量
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
    /*错误，有歧义，期望匹配和忽略的值不明确
    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second);
        }
    }*/

    //匹配守卫，match之后的额外if条件
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    //匹配守卫中使用逻辑运算符
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    //@绑定，把@右边所匹配的值绑定到@左边的变量上
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
        _ => println!("None"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    MutiColor(Color),
    Hello { id: i32 },
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
