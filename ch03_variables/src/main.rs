fn main() {
    //let mut
    {
        let mut x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");
    }

    //const
    {
        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    }

    //let
    {
        let y = 5;
        let y = y + 1;
        {
            let y = y * 2;
            println!("The value of y in the inner scope is: {y}");
        }
        println!("The value of y is: {y}");

        let spaces = "    ";
        let spaces = spaces.len();
        println!("{spaces}");
    }

    //f64,f32
    {
        let xx = 110.15;
        let yy: f32 = 100.0;
        let zz = xx - yy;
        println!("{zz}");
    }

    //tuple
    {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let tup2 = (500, 6.4, 1);
        let (x, y, z) = tup; //解构(destructuring)
        println!("The value of y is: {y}");
        let thrid = tup.2;
        let tup3 = ();
        println!("The value of z is: {thrid}");
    }

    //array
    {
        let a = [1, 2, 3, 4, 5];
        let b: [i32; 5] = [1, 2, 3, 4, 5];
        let c = [3; 5]; //[3,3,3,3,3]
        let first = a[0];
    }

    //func invoke
    {
        print_labeled_measurement(5, 'h');
        let x = five();
        println!("The value of x is: {x}");
    }

    //if
    {
        let number = 3;
        if number < 5 {
            println!("condition was true");
        } else {
            print!("condition was false");
        }
    }

    //loop
    {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        println!("The result is {result}");
    }

    //loop lable
    {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;
            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }
            count += 1;
        }
        println!("End count = {count}");
    }

    //while
    {
        let mut number = 3;
        while number != 0 {
            println!("{number}");
            number -= 1;
        }
        println!("LIFTOFF!!!");
    }

    //for
    {
        let a = [10, 20, 30, 40, 50];
        for element in a {
            println!("the value is: {element}");
        }
        for number in (1..4).rev() {
            println!("{number}");
        }
        println!("LIFTOFF!!!");
    }
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}
