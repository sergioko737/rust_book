use rand::Rng;

fn main() {
    let number = 3;

    if number < 5 {
    	println!("condition was true");
    } else {
    	println!("condition was false");
    }

    let condition = false;
    let condition = if condition { 5 } else { 6 };

    println!("The value of number is: {}", condition);


// loop example
    let mut counter = 0;

    let result = loop {
    	counter += 1;

    	let secret_number = rand::thread_rng().gen_range(1..101);

    	print!("{} ", secret_number);
    	if secret_number == 50 {
    		break counter;
    	}
    };

    println!("\nThe result is {}", result);

// while example
    let mut while_number = -1;

    while while_number != 50 {
    	while_number = rand::thread_rng().gen_range(1..101);
    	// while_number += 1;
    	print!("{} ", while_number);
    }

    println!("\nWe break out of while loop with: {}\n", while_number);

// for loop example
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
    	println!("the value is: {}", element)
    }

    for number in (1..4).rev() {
    	println!("{}!", number);
    }

}

