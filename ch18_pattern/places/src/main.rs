//匹配模式有两种形式：refutable(可反驳的)和irrefutable(不可反驳的)
//能匹配任何传递的可能值的模式称为不可反驳的，如let x = 5，x可以匹配任何值所以不可能失败；
//对某些可能的值进行匹配会失败的模式称为可反驳的，如if let Some(x) = a_value，若a_value是None则不匹配

fn main() {
    //match
    //必须使用可反驳模式，除了最后一个分支需要使用能匹配任何剩余值的不可反驳模式
    {
        let n: Option<i32> = None;
        let n: Option<i32> = match n {
            _ => None,
        };
    }
    //if let
    //接受可反驳和不可反驳模式
    {
        let n: Option<i32> = Some(1);
        if let Some(num) = n {
            println!("num = {}", num);
        }
    }
    //while let
    //接受可反驳和不可反驳模式
    {
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }
    //for
    //只接受不可反驳模式
    {
        let v = vec!['a', 'b', 'c'];
        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }
    //let
    //只接受不可反驳模式
    {
        let (x, y, z) = (1, 2, 3);
    }
    //function
    //只接受不可反驳模式
    {
        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: ({},{})", x, y);
        }
        let point = (3, 5);
        print_coordinates(&point);
    }
}
