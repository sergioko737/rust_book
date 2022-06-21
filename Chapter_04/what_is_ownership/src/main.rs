fn main() {
    let mut s = String::from("hello");

    println!("s is - {}", s);
    takes_ownership(&s);
    println!("s is - {}", s);

    {
        let r1 = &s;
        println!("r1 {}", r1);
        // let r1 = "yaya";
        println!("r1 {}", r1);
        println!("r1 {}", r1);
        let r2 = &mut s;
        // println!("r1 {}", r1); - r1 cannot be used after previous line as mutable borrowing has happened
        println!("r2 {}", r2);
    }

    // OUT OF SCOPE
    // println!("r1 {}", r1);
    // println!("r2 {}", r2);

    let x = 5;

    let z = makes_copy(x);

    println!("int after copy - {}", x);

    let mut count = 0;

    loop {
        if count > 5 {
            break;
        }
        println!("{}{}", x, count);
        count = count + 1;
    }

    println!("z is - {}", z);
    println!("z is - {}", z);
}

fn takes_ownership(some_string: &String) {
    println!("String - {}", some_string);
}

fn makes_copy(some_integer: i32) -> i32 {
    println!("int {}", some_integer);
    some_integer
}
