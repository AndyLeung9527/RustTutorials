use macro_derive::{make_answer, show_streams, PrintFn};

fn main() {
    //宏和函数的区别
    {
        //宏是元编程，在调用之前必须定义并引入作用域；而函数可以在任何地方定义和调用
    }

    //第一种宏的形式：声明宏，macro_rules!声明宏用于通用元编程
    {
        //vec!简化的定义
        //#[macro_export]注解表明只要导入定义宏的crate即可用，如果没有该注解则宏不能被引入作用域
        //macro_rules!和宏名称(不带感叹号)开始宏定义，名称后跟大括号表示宏定义体。该例中宏名称是vec
        //vec!宏结构和match表达式结构类似，($($x:expr),*)是其中的一个分支模式
        //1.一对圆括号包含整个模式
        //2.$()匹配符合括号内模式的值，在替代代码中使用，$x:expr匹配Rust的任意表达式，并将表达式命名为$x
        //3.逗号是可选分隔符，*说明之前的模式匹配零个或多个(比如vec![1,2,3]，$x模式会被匹配三次，3后既可以有逗号也可以没有)，调用该宏时，所生成的代码是：
        /*
        {
            let mut temp_vec = Vec::new();
            temp_vec.push(1);
            temp_vec.push(2);
            temp_vec.push(3);
            temp_vec
        }
         */
        #[macro_export]
        macro_rules! vec {
            ($($x:expr),*) => {
                {
                    let mut temp_vec = vec::new();
                    $(
                        temp_vec.push($x);
                    )*
                    temp_vec
                }
            };
        }
    }

    //第二种宏的形式：过程宏，用于从属性生成代码的过程宏
    //有三种类型：自定义派生(derive)，类属性、类函数
    //创建过程宏的时候，须定义在独立的包中，包名必须以derive为后缀，且包的类型也是特殊的
    {
        //自定义派生(derive)
        {
            //编写自定义derive宏(须单独新建一个包，由于两个crate紧密关联，可以直接在当前项目中新建)，看../macro_derive/src/lib.rs自定义派生(derive)定义部分
            //使用自定义derive宏()，先添加依赖，可以将macro_derive发布到crates.io上作为常规依赖；也可以通过指定path依赖，看../Cargo.toml
            //#[derive()]注解调用宏，只能用于结构体和枚举
            #[derive(PrintFn)]
            struct Pancakes;
            //须定义这个trait，derive会实现它执行，否则编译不通过
            trait PrintFn {
                fn print_name();
            }
            //调用
            Pancakes::print_name();
        }

        //类属性宏
        {
            //与自定义派生宏相似，不同的是derive属性生成代码，而类属性宏是创建新的属性
            //编写类属性宏(须单独新建一个包，由于两个crate紧密关联，可以直接在当前项目中新建)，看../macro_derive/src/lib.rs类属性宏定义部分
            //使用类属性宏，不仅限于结构体和枚举，还可用于函数等其他项
            #[show_streams]
            fn index() {}
        }

        //类函数宏
        {
            //编写类函数宏(须单独新建一个包，由于两个crate紧密关联，可以直接在当前项目中新建)，看../macro_derive/src/lib.rs类函数宏定义部分
            //使用类函数宏，像函数一样直接调用，可以接受未知数量的参数
            make_answer!();
            println!("{}", answer());
        }
    }
}
