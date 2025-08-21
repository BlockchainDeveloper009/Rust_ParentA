
use UtilsB::say_hello;
use E::say_hello_from_E;
use ApiServer::ConfigApiServer::get_params;

// fn main() {
//     println!("Hello, world! from Parent project A");
//     // example to call from Utils library
//     say_hello(); // Call function from Utils library
// // example to call function from E library
//     say_hello_from_E(); // Call function from E library

//     let params = get_params();
//     println!("Hello, world! from Parent project A");
//     println!("Port: {}", params.port);
//     println!("Host: {}", params.host);
//     println!("DB Host: {}", params.db_host);
//     println!("DB Port: {}", params.db_port);
//     println!("DB User: {}", params.db_user);
//     println!("DB Password: {}", params.db_password);
//     println!("DB Name: {}", params.db_name);
// }


fn main() {
    println!("Hello, world!");
    
    
    let n = 5;

    if (n < 0) {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well.
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon.
        };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);
    whileloop_example();
}

fn forAndRange_example(){ 
     // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
fn whileloop_example(){
 // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }

}
