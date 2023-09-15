//集成测试不能导入main.rs，只能导入crate，需要lib.rs
use adder; //tests目录中的测试文件是完全独立的crate，所以需要导入库

mod common; //定义新模块以便在所有集成测试文件中使用，使用mod.rs以不被当作测试文件

#[test]
fn it_added_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
