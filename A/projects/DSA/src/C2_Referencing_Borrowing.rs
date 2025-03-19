//references
/*
1. Shared
2. Mutable
3. Dangling
4. Borrowing
5. Lifetime
6. Copy
*/
pub fn change_string(some_string: &mut String) {
    some_string.push_str(", world");
}