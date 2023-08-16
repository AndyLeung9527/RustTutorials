fn main() {
    //Transfer of ownership
    {
        let s1 = String::from("hello");
        let len = calculate_length_1(s1);
        //println!("The length of '{s1}' is {len}.");//Error
    }

    //reference
    {
        let s1 = String::from("hello");
        let len = calculate_length_2(&s1);
        println!("The length of '{s1}' is {len}.");
    }

    //mutable reference
    {
        let mut s1 = String::from("hello");
        change_2(&mut s1);
    }

    {
        let mut s1 = String::from("hello");
        let r1 = &mut s1;
        //refer mut many times in the same scope
        //let r2 = &mut s1;//Error
    }

    {
        /* 错误, 同时拥有可变引用和不可变引用
        let mut s1 = String::from("hello");
        let r1 = &s1;
        let r2 = &s1;
        let r3 = &mut s1;
        println!("{r1},{r2},{r3}");
        */

        /* 正确,不可变引用作用域结束后(所有权转移)才创建可变引用 */
        let mut s1 = String::from("hello");
        let r1 = &s1;
        let r2 = &s1;
        println!("{r1},{r2}");
        let r3 = &mut s1;
        println!("{r3}");
    }

    //string slice
    {
        let s = String::from("hello world");
        let hello = &s[0..5]; //&s[..5];
        let world = &s[6..11]; //&s[6..];
    }

    {
        let mut s = String::from("hello world");
        let word = first_word(&s);
        //错误, 同时拥有可变引用和不可变引用
        //s.clear();//Error
        println!("the first word is: {word}");
    }

    //array slice
    {
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        assert_eq!(slice, &[2, 3]);
    }
}

fn calculate_length_1(s: String) -> usize {
    s.len()
}

fn calculate_length_2(s: &String) -> usize {
    s.len()
}

fn change_1(some_string: &String) {
    //modifying is not allowed
    //some_string.push_str(", world");//Error
}

fn change_2(some_string: &mut String) {
    some_string.push_str(", world");
}

/* Dangling references
fn dangle() -> &String {
    let s = String::from("hello");
    &s//Error
}
*/

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
