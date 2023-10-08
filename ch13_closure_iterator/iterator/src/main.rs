fn main() {
    let v1 = vec![1, 2, 3];

    //创建v1的迭代器
    //1：迭代返回不可变引用，使用iter()；
    //2：迭代返回所有权，使用into_iter();
    //3：迭代返回可变引用，使用iter_mut();
    let v1_iter = v1.iter();
    //for会获取迭代器所有权，无需可变
    for val in v1_iter {
        println!("Got: {}", val);
    }

    //创建可变的v1的迭代器
    let mut v1_iter = v1.iter();
    //直接调用next方法，会改变迭代器记录序列位置的状态，所以是可变的
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    //let total2: i32 = total + v1_iter.sum();//Error，迭代器所有权已转移

    //迭代器的map方法，使用闭包遍历元素并执行操作
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        let in_my_size = shoes_in_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }
}
