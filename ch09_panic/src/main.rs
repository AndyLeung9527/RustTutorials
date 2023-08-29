use std::error::Error;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};

//main函数也可以返回Result<(),E>,Box<dyn Error>任何类型的错误,Ok以0值退出,Err以非0值退出
//main函数也可以返回任何实现了std::process::Termination(trait)的类型,它包含了一个返回ExitCode的report函数
fn main() -> Result<(), Box<dyn Error>> {
    //Match Result<T,E>
    {
        let greeting_file_result = File::open("hello.txt");
        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            },
        };
    }

    //unwrap,Ok则返回其中的值,Err则调用panic!宏,使用默认panic!信息
    {
        let greeting_file = File::open("world.txt").unwrap();
    }

    //expect,Ok则返回其中的值,Err则调用panic!宏,panic!信息从参数中传递
    {
        let greeting_file =
            File::open("world.txt").expect("world.txt should be included in this project");
    }

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

//?,如果值是Ok,则返回Ok的值继续执行,如果是Err,则返回Err和其中的值
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

//链式
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

//使用fs::read_to_string
fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

//?同样适用Opiton<T>,如果值是Some,则返回Some的值继续执行,如果是None,则返回
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

//通过创建新类型来检查值
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value: value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}
