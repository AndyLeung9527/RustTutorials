//需要指定到内部引用
// use pubuse::kinds::PrimaryColor;
// use pubuse::utils::mix;

//使用pub use重导出到顶层后，可以直接在顶层引用
use pubuse::mix;
use pubuse::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
