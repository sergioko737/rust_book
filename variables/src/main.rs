fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let fx = 12;

    let fy = 5;
    
    println!("fx: {}, fy:", fx&fy);

    let xtuple: (i32, f64, u8) = (500, 6.4, 1);

    println!("tuple is {} {} {}", xtuple.0, xtuple.1, xtuple.1);

    non_polymorphic_func(&xtuple.0 + 3);

}



fn non_polymorphic_func(param: i32) {
	println!("The value of param: {}", param);
}