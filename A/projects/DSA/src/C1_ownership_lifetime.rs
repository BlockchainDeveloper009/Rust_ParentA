

pub fn take_ownership(v: Vec<String>) {
    println!("{:?}", v);
}

pub fn make_copy(one: i32) {
    let val1 = one;
    println!("one: {}", val1);
}

pub fn give_ownership() -> String {
    let s = String::from("hello");
    s
}
pub fn take_and_give_back(s: String) -> String {
    s
}

struct MyString<'a>{
    text: &'a str,

}
pub fn lifeTime_example() {
    let str1 = String::from("hello");
    //now by using the reference of text we can use it in the struct
    // by giving life time, we are telling, MyString instance cannot out live text
    let my_string = MyString{text: str1.as_str()};
    println!("my_string: {}", my_string.text);

    let s: &'static str = "I have a static lifetime.";
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifeTime_example() {
    }
}