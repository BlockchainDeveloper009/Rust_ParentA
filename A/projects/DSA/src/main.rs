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

fn vector_main() {
    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("{:?}", numbers); // Output: [1, 2, 3]

     let fruits = vec!["apple", "banana", "cherry"];
    for fruit in &fruits {
        println!("{}", fruit);
    }

        //Accessing elements

    let names = vec!["Alice", "Bob", "Charlie"];
    println!("First name: {}", names[0]); // Direct indexing
    match names.get(2) {
        Some(name) => println!("Third name: {}", name),
        None => println!("No third name found"),
    }

    //Removing Elements:

     let mut items = vec![10, 20, 30, 40];
    items.remove(1); // Removes the element at index 1 (20)
    println!("{:?}", items); // Output: [10, 30, 40]


//Iterating & Modifying:


    let mut scores = vec![10, 20, 30];
    for score in &mut scores {
        *score += 5;
    }
    println!("{:?}", scores); // Output: [15, 25, 35]


    let mut nums = vec![1, 2, 3, 4, 5];
nums.retain(|&x| x % 2 == 0); // Keeps only even numbers
println!("{:?}", nums); // [2, 4]


}

struct Person {
    name: String,
    age: u32,
}

fn Vectors_with_strucs() {
    let people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
    ];

    for person in &people {
        println!("{} is {} years old", person.name, person.age);
    }
}

