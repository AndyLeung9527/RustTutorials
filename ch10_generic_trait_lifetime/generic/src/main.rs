fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let result = largest(&number_list);
    let result = largest(&char_list);

    let integer = Point { x: 5, y: 10 };
    //let wont_work = Point { x: 5, y: 4.0 };//Error,x与y类型不一致
    let wont_work = Point_2 { x: 5, y: 4.0 };

    integer.x();

    let p1 = Point_2 { x: 5, y: 10.4 };
    let p2 = Point_2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//泛型函数
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//泛型结构体
struct Point<T> {
    x: T,
    y: T,
}

//多个泛型结构体
struct Point_2<T, U> {
    x: T,
    y: U,
}

//泛型方法
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

//泛型方法泛型指定限制
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

//泛型方法签名类型可与结构体泛型类型不一致
impl<T, U> Point_2<T, U> {
    fn mixup<T2, U2>(self, other: Point_2<T2, U2>) -> Point_2<T, U2> {
        Point_2 {
            x: self.x,
            y: other.y,
        }
    }
}
