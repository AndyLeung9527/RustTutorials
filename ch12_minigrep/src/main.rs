use minigrep::Config;
use std::env; //注意std::env::args在其任何参数包含无效Unicode字符时会panic
use std::process;

//分别执行下列指令(powershell)，以设置环境变量和执行
//$Env:IGNORE_CASE=1;
//cargo run -- to E:\LWB_Projects\RustTutorial\ch12_minigrep\poem.txt
//执行下列指令(powershell)移除环境变量
//Remove-Item Env:IGNORE_CASE
//分别执行下列指令，标准输出会重定向到output.txt，错误输出会打印到标准错误流
//cargo run > output.txt
//cargo run -- to E:\LWB_Projects\RustTutorial\ch12_minigrep\poem.txt > output.txt
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        //println!("Problem parsing arguments:{err}");
        eprintln!("Problem parsing arguments:{err}"); //打印到标准错误流
        process::exit(1);
    });

    //build()在十三章中优化为使用迭代器实现
    let config = Config::build_from_iterator(env::args()).unwrap_or_else(|err| {
        //println!("Problem parsing arguments:{err}");
        eprintln!("Problem parsing arguments:{err}"); //打印到标准错误流
        process::exit(1);
    });

    println!(
        "Searching for {} in file {}",
        config.query, config.file_path
    );

    if let Err(e) = minigrep::run(config) {
        //println!("Application error:{e}");
        eprintln!("Application error:{e}"); //打印到标准错误流
        process::exit(1);
    }
}
