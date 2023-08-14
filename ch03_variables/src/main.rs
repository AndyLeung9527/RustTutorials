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
}
