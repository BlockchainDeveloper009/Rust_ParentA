fn get_name() -> &str {
    let name = String::from("Alice"); // `name` is created inside the function
    &name // ❌ ERROR: Borrowing a local variable that will be dropped at the end of the function
}

fn main() {
    let name_ref = get_name();
    println!("{}", name_ref); // This would be a dangling reference!
}

struct Person {
    name: &str, // ❌ ERROR: Missing lifetime annotation
}

fn main() {
    let p;
    {
        let name = String::from("Alice");
        p = Person { name: &name }; // ❌ ERROR: `name` is deallocated after this block
    }
    println!("{}", p.name); // ❌ Dangling reference!
}

//======function returning struct

fn create_person() -> Person {
    let name = String::from("Alice");
    Person { name: &name } // ❌ ERROR: `name` is deallocated after function ends!
}

fn main() {
    let p = create_person();
    println!("{}", p.name); // ❌ Dangling reference!
}
