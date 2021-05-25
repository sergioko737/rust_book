fn main() {
    
    struct User {
    	username: String,
    	email: String,
    	sign_in_count: u64,
    	active: bool,
    }

    let mut user1 = User {
    	email: String::from("user1@mail.com"),
    	username: String::from("username1"),
    	active: true,
    	sign_in_count: 1,
    };

    // 
    println!("Here goes user1: {}", &user1.email);

    user1.email = String::from("user1@newmail.com");
    println!("Here goes user1: {}", user1.email);

    // user builder func
    fn build_user(email: String, username: String) -> User {
	    User {
	        email,
	        username,
	        active: true,
	        sign_in_count: 1,
	    }
	}

    let user2 = build_user("user2@mail.com".to_string(), "username2".to_string());

    println!("user2 mail: {}, user2 name: {}", user2.email, user2.username);

    let user3 = User {
    	email: String::from("user3@mail.com"),
    	username: String::from("username3"),
    	..user1
    };

    println!("user3 mail: {}, user3 name: {}", user3.email, user3.username);


    // struct tuples
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black details: {}, {}, {}", black.0, black.1, black.2);

    let green = black;

    // below line will fail because black moved to green
    // println!("black details: {}, {}, {}", black.0, black.1, black.2); 
    println!("green details: {}, {}, {}", green.0, green.1, green.2);


}


