mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    pub mod serving {
        pub fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
fn eat_at_restaurant() {
    //绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    //相对路径
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        //super从父模块开始构建相对路径
        super::deliver_order();
    }
    fn cook_order() {}
}

mod back_of_house_2 {
    //公有结构成员默认还是私有
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
fn eat_at_restaurant_2() {
    let mut meal = back_of_house_2::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    //println!("I'd like {} toast please", meal.seasonal_fruit);//Error
}

mod back_of_house_3 {
    //公有枚举成员也是公有
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
pub fn eat_at_restaurant_3() {
    let order1 = back_of_house_3::Appetizer::Soup;
    let order2 = back_of_house_3::Appetizer::Salad;
}

//使用use简化调用
use crate::front_of_house::hosting;
pub fn eat_at_restaurant_4() {
    hosting::add_to_waitlist();
}
/* Error,超出use作用域,需要在mod里use
mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
*/

use std::fmt;
use std::io;
fn function1() -> fmt::Result {
    Ok(())
}
fn function2() -> io::Result<()> {
    Ok(())
}

use std::fmt::Result;
use std::io::Result as IoResult; //通过as重命名解决冲突
fn function3() -> Result {
    Ok(())
}
fn function4() -> IoResult<()> {
    Ok(())
}

pub use crate::front_of_house::serving; //pub use重定向
pub fn eat_at_restaurant_5() {
    serving::take_order();
}

mod a {
    /*
    use std::cmp::Ordering;
    use std::io;
    */
    use std::{cmp::Ordering, io}; //一行引入多个
}

mod b {
    /*
    use std::io;
    use std::io::Write;
    */
    use std::io::{self, Write};
}

mod c {
    use std::collections::*; //glob运算符,引入所有
}
