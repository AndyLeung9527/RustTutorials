fn main() {
    //struct
    {
        let mut user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };
        user1.email = String::from("anotheremail@example.com");

        let user2 = User {
            email: String::from("another@example.com"),
            ..user1
        };

        //失败,String所有权转移
        //let user1name = user1.username;//Error
        //成功
        let user1active = user1.active;
    }

    //tuple struct
    {
        let black = Color(0, 0, 0);
    }

    //unit-like struct
    {
        let subject = AlwaysEqual;
    }

    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect1)
        );
        println!("rect1 is {:?}, rect1 is {:#?}", rect1, rect1);
    }

    {
        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };
        dbg!(&rect1);
    }

    //method
    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }

    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }

    {
        let sq = Rectangle::square(3);
    }
}

fn build_user(email: String, username: String) -> User {
    /*
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
    */
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//tuple struct
struct Color(i32, i32, i32);

//unit-like struct
struct AlwaysEqual;

//外部属性,用于打印出调试信息
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    //associated function(关联函数)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //同样是associated function(关联函数),第一参数不为self,所以并不作用于实例
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
