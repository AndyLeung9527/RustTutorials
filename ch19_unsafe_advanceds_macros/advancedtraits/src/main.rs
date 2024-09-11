use std::fmt;

fn main() {
    //关联类型在trait定义中指定占位符类型
    {
        struct Counter;
        //标准库的Iterator例子
        pub trait Iterator {
            type Item; //占位符类型，类似泛型
            fn next(&mut self) -> Option<Self::Item>; //由实现者指定具体类型
        }
        //实现Iterator trait，指定Item类型为u32
        //与泛型区别是，使用泛型可以多次实现这个trait；而使用关联类型只能选择一次Item类型，调用时无须指定类型
        impl Iterator for Counter {
            type Item = u32;
            fn next(&mut self) -> Option<Self::Item> {
                Option::None
            }
        }
    }

    //默认泛型类型参数和运算符重载
    {
        struct Millimeters(u32);
        struct Meters(u32);
        //标准库的Add例子
        //Rhs=Self语法叫做默认类型参数，Rhs(right hand side)用于定义add方法中的rhs参数类型，若实现时不指定具体类型，则默认Self类型
        trait Add<Rhs = Self> {
            type Output;
            fn add(self, rhs: Rhs) -> Self::Output;
        }
        //实现Millimeters与Meters相加，Add<Meters>设定Rhs类型参数为Meters而非默认的Self
        impl Add<Meters> for Millimeters {
            type Output = Millimeters;
            fn add(self, rhs: Meters) -> Self::Output {
                Millimeters(self.0 + (rhs.0 * 1000))
            }
        }
    }

    //完全限定语法与消歧义：调用相同名称的方法
    {
        //关联函数带self(Rust通过实例计算使用哪个trait)
        {
            struct Human;
            trait Pilot {
                fn fly(&self);
            }
            trait Wizard {
                fn fly(&self);
            }
            impl Pilot for Human {
                fn fly(&self) {
                    println!("This is your captain speaking.");
                }
            }
            impl Wizard for Human {
                fn fly(&self) {
                    println!("Up!");
                }
            }
            impl Human {
                fn fly(&self) {
                    println!("*waving arms furiously*");
                }
            }

            let person = Human;
            //调用Human实例的fly
            person.fly();
            //另一种写法
            Human::fly(&person);
            //调用Pilot的fly
            Pilot::fly(&person);
            //调用Wizard的fly
            Wizard::fly(&person);
        }

        //关联函数不带self(Rust无法计算使用哪个trait，须使用完全限定语法)
        {
            struct Dog;
            trait Animal {
                fn baby_name() -> String;
            }
            impl Animal for Dog {
                fn baby_name() -> String {
                    String::from("Spot")
                }
            }
            impl Dog {
                fn baby_name() -> String {
                    String::from("puppy")
                }
            }
            //调用Dog上的关联函数
            println!("A baby dog is called a {}", Dog::baby_name());
            //println!("A baby dog is called a {}", Animal::baby_name());//Error，Rust无法计算使用哪个实现
            //调用Animal的实现，使用完全限定语法：<Type as Trait>::function(receiver_if_method, next_arg, ...)
            println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
        }
    }

    //父(超)trait
    {
        struct Point {
            x: i32,
            y: i32,
        }
        trait OutLinePrint: fmt::Display {
            fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
            }
        }
        //impl OutLinePrint for Point {}//Error，Display未被实现
        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }
        impl OutLinePrint for Point {} //Correct，Display已实现
        let point = Point { x: 1, y: 1 };
        point.outline_print();
    }

    //newtype模式用以在外部类型上实现外部trait
    {
        //impl fmt::Display for Vec<String> {}//Error，在Vec<T>上实现Display不被孤儿规则所允许，因为Display trait和Vec<T>定义在crate之外
        //通过创建一个包含Vec<T>实例的Wrapper结构体，再实现Display
        struct Wrapper(Vec<String>);
        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "[{}]", self.0.join(", "))
            }
        }
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {w}");
    }
}
