use std::fmt::{Debug, Display};

//trait的定义
pub trait Summary {
    fn summarize_author(&self) -> String;
    //trait的默认实现
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

//trait作为参数(语法糖)
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item1.summarize());
}
//trait作为参数(非语法糖,可限制参数类型相同)
pub fn notify_bound<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
}
//多类trait作为参数(语法糖)
pub fn notify_multiple(item: &(impl Summary + Display)) {}
//多类trait作为参数(非语法糖,可限制参数类型相同)
pub fn notify_bound_multiple<T: Summary + Display>(item: &T) {}
//多类trait作为参数(非语法糖,where限制)
pub fn some_function<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

//返回trait类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
//Error,返回trait只能返回单一类型
/* fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}
 */

struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

//根据trait在泛型上实现方法
impl<T: Display> Summary for T {
    fn summarize_author(&self) -> String {
        format!("@{}", &self)
    }
}
