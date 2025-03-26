fn get_name<'a>(name: &'a str) -> &'a str {
    name // Return the same reference that was passed in
}

fn main() {
    let my_name = String::from("Alice");
    let name_ref = get_name(&my_name); // Pass reference from outer scope
    println!("{}", name_ref); // ✅ Safe! The reference is valid.
}

//-- structs with life time

struct Person<'a> {
    // ✅ Define a lifetime `'a`
    name: &'a str, // ✅ Reference must live as long as `'a`
}

fn main() {
    let name = String::from("Alice");
    let p = Person { name: &name }; // ✅ Safe: `name` lives long enough
    println!("{}", p.name);
}

//==== function returning struct code

fn create_person<'a>(name: &'a str) -> Person<'a> {
    Person { name } // ✅ Now lifetime `'a` ensures name lives long enough
}

fn main() {
    let name = String::from("Alice");
    let p = create_person(&name); // ✅ Safe reference
    println!("{}", p.name);
}
