pub mod log_mod;
use log_mod::logger::write_log;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn say_hello() {
    println!("Hello from the Utils library!");
    write_log("call from lib.rs file:say_hello method ");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        say_hello();
        assert_eq!(result, 4);
    }

    
    #[test]
    fn it_write_log() {
        let result = write_log("test call from lib.rs file ");
        
    }
}
