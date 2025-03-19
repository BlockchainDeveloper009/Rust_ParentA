pub mod c1_ownership_lifetime;
pub mod c2_Referencing_Borrowing;
// Compare this snippet from A/projects/DSA/src/c2_Referencing_Borrowing.rs:
use c1_ownership_lifetime::take_ownership;
use c1_ownership_lifetime::{make_copy,give_ownership,take_and_give_back};
use c2_Referencing_Borrowing::change_string;

fn main() {

    println!("Hello, world!");

    let var = 5;
    let mut var2 = 6;
    let mut s = "hello".to_string();
    s.push_str(" world");
    println!("s: {}", s);

    let x = vec!["tyler".to_string(), "mike".to_string()];
    println!("{:?}", x);
    let y = x;

    println!("{:?}", y);

    take_ownership(y);

    let val = 1;
    make_copy(val);
    println!("val: {}", val);
    let str1:String = give_ownership();
    println!("str1: {}", str1);

    let str2 = take_and_give_back(str1);
    println!("str2: {}", str2);
    let mut changing_string = String::from("hello");
    change_string(&mut changing_string);
    println!("changing_string: {}", changing_string);

    // let mut r=9;
    // println!("r: {}", r);
    // {
    //     let x = 5;
    //     //r = &x; wrong code
    //     r = *x; // correct code
    // } // x is dropped, hence dangling reference
    // println!("r: {}", r);

}


