fn main() {
    //init
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = String::from("initial contents");

    //push_str
    let mut s = String::from("foo");
    s.push_str("bar");

    //s2内容附加到s1后还能使用
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    //push
    let mut s = String::from("lo");
    s.push('l');

    //+
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; //fn add(self, s: &str) -> String {
    //println!("{s1}");//Error
    println!("{s2}");
    println!("{s3}");

    //format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");

    //字符串不支持索引
    let s1 = String::from("hello");
    //let h = s1[0];//Error

    //slice
    let hello = "Здравствуйте";
    let s = &hello[0..4]; //字节
    //Panic error,一个字符占两字节
    //let s1 = &hello[0..1];
    println!("{s}");

    //遍历字符
    for c in "Зд".chars() {
        println!("{c}");
    }

    //遍历字节
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
