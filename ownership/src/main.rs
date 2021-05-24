fn main() {
    let s = String::from("hello");
    let x = 5;

    println!("{:?}", s);
    println!("one more time {:?}", s);
    some_print(s, x);
    println!("and again {}", x);

    let mut sx = String::from("privet");

    let r1 = &sx; // no problem
    let r2 = &sx; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    println!("{} and {}", r1, r2);
    let r3 = &mut sx; // no problem
    println!("{}", r3);
    // next line if uncommented will cause a compilation problem
    // because mutable borrow already happened at line 18
    // println!("{} and {}", r1, r2);

}

fn some_print(some_string: String, some_int: i32) {
	println!("print from func: String {:?} and int {}", some_string, some_int);
}