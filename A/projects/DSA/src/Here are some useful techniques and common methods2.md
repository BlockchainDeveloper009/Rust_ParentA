Here are some useful techniques and common methods to handle **strings** and **string arrays (collections)** in Rust, especially relevant for algorithms, dynamic programming, and general text processing:

***

## 1. String Creation and Conversion
- Create owned `String` or borrowed string slices `&str`.
- Convert between them with `.to_string()` or `.as_str()`.

```rust
let s: String = String::from("hello");
let slice: &str = &s;
let s2 = "world".to_string();
```

***

## 2. String Concatenation and Interpolation
- Use `+` operator or `push_str()` to append.
- For formatted strings with variables, use `format!()` macro.

```rust
let mut s = String::from("hello");
s.push_str(" world");  // s = "hello world"

let name = "Alice";
let greeting = format!("Hi, {}!", name); // interpolation
```

***

## 3. Iterating Characters
- Strings are UTF-8, so iterate over chars safely with `.chars()`.

```rust
for ch in "Rust🦀".chars() {
    println!("{}", ch);
}
```

***

## 4. Slicing Strings
- Use slices &str without copying.
- Be mindful of character boundaries (UTF-8).

```rust
let s = "hello";
let h = &s[0..1]; // "h"
let ell = &s[1..4]; // "ell"
```

***

## 5. Searching and Pattern Matching
- Check substring presence with `.contains()`.
- Find substring indices using `.find()` which returns `Option<usize>`.
- Check prefixes/suffixes with `.starts_with()`, `.ends_with()`.

```rust
let text = "hello rust";
if text.contains("rust") {
    println!("Found 'rust'");
}
if let Some(i) = text.find("rust") {
    println!("Index: {}", i);
}
```

***

## 6. Replacing Strings
- Replace substrings globally with `.replace()`.
- Use `.replacen()` for limited number of replacements.

```rust
let s = "foo bar foo";
let new_s = s.replace("foo", "baz");  // "baz bar baz"
let new_s_once = s.replacen("foo", "baz", 1); // "baz bar foo"
```

***

## 7. Splitting and Joining Strings
- Split strings by delimiters or whitespace using `.split()` and `.split_whitespace()`.
- Collect split results into a vector (e.g., `Vec<&str>`).
- Join string slices with `.join()`.

```rust
let sentence = "apple,banana,grape";
let fruits: Vec<&str> = sentence.split(',').collect();
println!("{:?}", fruits); // ["apple", "banana", "grape"]

let joined = fruits.join(" & ");
println!("{}", joined); // "apple & banana & grape"
```

***

## 8. Trimming Whitespace
- Use `.trim()`, `.trim_start()`, `.trim_end()` to strip unwanted spaces.

```rust
let padded = "  hello  ";
println!("'{}'", padded.trim()); // 'hello'
```

***

## 9. Working With String Arrays (Vec<String>)
- Create and manipulate vectors of strings.

```rust
let mut words = vec!["hello".to_string(), "world".to_string()];
words.push("Rust".to_string());
for word in &words {
    println!("{}", word);
}
```

***

## 10. Reversing Strings
- Use iterators `.chars().rev()` to reverse strings safely considering UTF-8.

```rust
let s = "hello";
let rev: String = s.chars().rev().collect();
println!("{}", rev); // "olleh"
```

***

### Summary Table of Useful String Methods in Rust

| Task                   | Method(s)                   | Description                        |
|------------------------|-----------------------------|----------------------------------|
| Create/Convert         | `String::from()`, `.to_string()`   | Create owned or convert slices    |
| Concatenate            | `+`, `.push_str()`, `format!()`| Append or interpolate strings     |
| Iterate chars          | `.chars()`                   | Unicode-safe iteration            |
| Slice                  | `&str[start..end]`           | Substring slicing (UTF-8 safe)   |
| Search                 | `.contains()`, `.find()`     | Check substring presence/index   |
| Replace                | `.replace()`, `.replacen()`  | Substitute substrings             |
| Split                  | `.split()`, `.split_whitespace()` | Split by delimiter or whitespace |
| Join                   | `.join()`                    | Combine substrings                |
| Trim                   | `.trim()`, `.trim_start()`, `.trim_end()` | Remove whitespace                |
| Reverse                | `.chars().rev().collect()`   | Reverse string safely            |

***

Learning these basics and methods will make your string handling in Rust efficient, safe, and idiomatic. If you want, I can provide code examples or help with specific string challenges!

[1](https://dev.to/alexmercedcoder/in-depth-guide-to-working-with-strings-in-rust-1522)
[2](https://codesignal.com/learn/courses/string-manipulation-in-rust/lessons/advanced-string-methods-in-rust)
[3](https://www.tutorialspoint.com/rust/rust_string.htm)
[4](https://doc.rust-lang.org/rust-by-example/std/str.html)
[5](https://ezesunday.com/blog/rust-string-manipulation/)
[6](https://www.reddit.com/r/rust/comments/189a5tu/string_manipulation_in_rust_advent_of_code/)
[7](https://users.rust-lang.org/t/string-manipulation/96821)
[8](https://www.youtube.com/watch?v=GK9Iz_ihmV8)
[9](https://friendlyuser.github.io/posts/tech/2023/A_Dive_into_Rust's_String_Manipulation_Libraries_for_Text_Processing/)