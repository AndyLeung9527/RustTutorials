/*
三斜杠文档注释支持Markdown注解来格式化文本，位于需要文档注释的项之前
文档注释会生成HTML文档，展示公有API文档注释的内容，用于解释如何使用这个crate
*/
/*
如下crate中的add_one函数的文档注释，展示如何使用add_one函数的代码
可以运行cargo doc来生成这个文档注释的HTML文档，此命令由rustdoc工具生成HTML文档并放入target/doc目录
*/
/*
经常在文档注释中使用的部分：
Panics：这个函数可能会panic!的场景。并不希望程序崩溃的函数调用者应该确保他们不会在这些情况下调用此函数
Errors：如果这个函数返回Result，此部分描述可能会出现何种错误以及什么情况会造成这些错误，这有助于调用者编写代码来采用不同的方式处理不同的错误
Safety：如果这个函数使用unsafe代码，这一部分应该会涉及到期望函数调用者支持的确保unsafe块中代码正常工作的不变条件(invariants)
*/
/*
cargo test会测试文档注释中的示例代码(Doc-tests)
*/
/*
//!开头的注释用于crate根文件或模块的根文件，用来描述整个crate或模块
*/

//! # crates
//!
//! `crates` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = crates::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/*
发布crate到crate.io：
需要在crates.io上注册账号并获取一个API token
使用API token运行cargo login命令：cargo login abcdefghijklmnopqrstuvwxyz012345
命令会将API token存储在本地~/.cargo/credentials文件中
*/
/*
发布前向crate添加元信息：
crate名称，Cargo.toml中[package]的name，名称必须唯一，否则发布时会报错
license标识符值，Linux基金会的Software Package Data Exchange (SPDX)列出了可以使用的标识符，例如使用MIT或与Rust自身相同的双许可的MIT OR Apache-2.0，其中OR是指定多个license的分隔符；
若使用文本license，需将该文件包含进项目，然后使用license-file字段来指定文件名而非license字段
更多的元信息查看cargo文档
结果如下，文件名：Cargo.toml
[package]
name = "andyleung9527_demo_crates"
version = "0.1.0"
edition = "2021"
description = "A demo crate."
license = "MIT OR Apache-2.0"
*/
/*
发布：
发布是永久性的，对应版本不会被覆盖，代码不会被删除，这样所有依赖crates.io中的crate的项目都能一直正常工作
执行cargo publish命令进行发布
*/

/*
发布新版本：
修改Cargo.toml中version指定的值，使用语义化版本号规则，接着执行cargo publish上传新版本
*/

/*
撤回弃用版本：
撤回某个版本会阻止新项目依赖此版本，不过现存此依赖的项目仍然能够下载和依赖整个版本。
撤回，在项目里执行cargo yank --vers 0.1.0
撤销撤回，在项目里执行cargo yank --vers 0.1.0 --undo
撤回并没有删除任何代码
*/

/*
cargo install命令用于在本地安装和使用二进制crate，这个操作不会替换已下载本地的crate包
二进制目标文件是在crate有src/main.rs或者其他指定为二进制文件时所创建的可执行程序
cargo install把二进制文件安装到Rust安装根目录的bin文件夹中(若Rust是通过rustup.rs进行安装，则目录是$HOME/.cargo/bin，并且目录需要存在于$PATH环境变量中)
示例：cargo install ripgrep
最后两行：
Installing C:\Users\LWB\.cargo\bin\rg.exe
Installed package `ripgrep v13.0.0` (executable `rg.exe`)
倒数第二行说明安装目录，最后一行说明执行程序运行指令
*/
/*
cargo uninstall命令卸载，示例：cargo uninstall ripgrep
*/
/*
这个设计使得开发者可以通过新的子命令来对cargo进行扩展：
若$PATH中有类似cargo-something的二进制文件，则可通过cargo something来运行，类似这样的自定义命令可以运行cargo --list展示出来
*/