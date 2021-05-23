fn main() {
    let s = String::from("hello");
    let x = 5;

    println!("{:?}", s);
    println!("one more time {:?}", s);
    some_print(s, x);
    println!("and again {}", x);


}

fn some_print(some_string: String, some_int: i32) {
	println!("print from func: String {:?} and int {}", some_string, some_int);
}