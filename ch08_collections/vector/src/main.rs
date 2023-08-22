fn main() {
    //新建
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v: Vec<i32> = Vec::new();

    //更新
    v.push(5);
    v.push(6);
    v.push(7);

    //读取
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    let third: Option<&i32> = v.get(100);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    //遍历
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
        //v.push(0);//Error,多个可变引用(多次借用)
    }

    //通过枚举实现vec存储不同类型的值
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
