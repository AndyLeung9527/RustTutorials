use std::fmt::Display;

fn main() {
    {
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xz");
            result = longest_2(string1.as_str(), string2.as_str()); //longest(string1.as_str(), string2.as_str());//Error
        }
        println!("The longest string is {}", result);
    }

    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcept {
            part: first_sentence,
        };
    }

    {
        //静态生命周期，能够存活于整个程序期间
        let s: &'static str = "I have a static lifetime.";
    }
}

//生命周期方法
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

/*
每个引用都有生命周期，符合某些规则情况下(编译器能自动计算出所有输入输出参数的生命周期下)可以省略不标注
生命周期省略:
第一条规则是编译器为每一个引用参数都分配一个生命周期参数。换句话说就是，函数有一个引用参数的就有一个生命周期参数：fn foo<'a>(x: &'a i32)，有两个引用参数的函数就有两个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推。
第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。
第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法 (method)，那么所有输出生命周期参数被赋予 self 的生命周期。
例子1:
fn first_word(s: &str) -> &str {
编译器应用第一条规则后
fn first_word<'a>(s: &'a str) -> &str {
继续应用第二条规则后
fn first_word<'a>(s: &'a str) -> &'a str {
不符合第三条规则，结束(编译通过)
例子2
fn longest(x: &str, y: &str) -> &str {
编译器应用第一条规则后
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
不符合第二、三条规则，结束(编译不通过)
*/

//生命周期结构体
struct ImportantExcept<'a> {
    part: &'a str,
}
impl<'a> ImportantExcept<'a> {
    //符合第一条规则，self引用的生命周期可省略
    fn level(&self) -> i32 {
        3
    }
    //符合第三条规则，&self和announcement有各自的生命周期，输出参数被赋予&self生命周期，生命周期可省略
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

//结合泛型、trait bounds和生命周期
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
