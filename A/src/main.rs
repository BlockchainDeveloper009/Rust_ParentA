
use UtilsB::say_hello;
use E::say_hello_from_E;
use ApiServer::ConfigApiServer::get_params;

fn main() {
    println!("Hello, world! from Parent project A");
    // example to call from Utils library
    say_hello(); // Call function from Utils library
// example to call function from E library
    say_hello_from_E(); // Call function from E library

    let params = get_params();
    println!("Hello, world! from Parent project A");
    println!("Port: {}", params.port);
    println!("Host: {}", params.host);
    println!("DB Host: {}", params.db_host);
    println!("DB Port: {}", params.db_port);
    println!("DB User: {}", params.db_user);
    println!("DB Password: {}", params.db_password);
    println!("DB Name: {}", params.db_name);
}
