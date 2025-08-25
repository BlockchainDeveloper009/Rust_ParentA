Here are different techniques to perform **bitwise operations** in Rust with examples and their outputs:

***

### 1. Bitwise AND (`&`)

Perform boolean AND on each bit:

```rust
fn main() {
    let a: u8 = 0b1100; // 12 decimal
    let b: u8 = 0b1010; // 10 decimal

    let result = a & b;

    println!("Bitwise AND: {:04b} (decimal: {})", result, result);
}
```

**Output:**

```
Bitwise AND: 1000 (decimal: 8)
```

***

### 2. Bitwise OR (`|`)

Boolean OR on bits:

```rust
fn main() {
    let a: u8 = 0b1100;
    let b: u8 = 0b1010;

    let result = a | b;

    println!("Bitwise OR: {:04b} (decimal: {})", result, result);
}
```

**Output:**

```
Bitwise OR: 1110 (decimal: 14)
```

***

### 3. Bitwise XOR (`^`)

Exclusive OR:

```rust
fn main() {
    let a: u8 = 0b1100;
    let b: u8 = 0b1010;

    let result = a ^ b;

    println!("Bitwise XOR: {:04b} (decimal: {})", result, result);
}
```

**Output:**

```
Bitwise XOR: 0110 (decimal: 6)
```

***

### 4. Bitwise NOT (`!`)

Invert all bits:

```rust
fn main() {
    let a: u8 = 0b1100;

    let result = !a;

    println!("Bitwise NOT: {:08b} (decimal: {})", result, result);
}
```

**Output:**

```
Bitwise NOT: 11110011 (decimal: 243)
```

*(Note: Since u8 is 8 bits, leading bits shown. The decimal is unsigned, so NOT flips bits within 8 bits.)*

***

### 5. Left Shift (`<<`)

Shift bits left (multiply by 2 each shift):

```rust
fn main() {
    let a: u8 = 0b0001; // 1

    let result = a << 3;

    println!("Left Shift: {:08b} (decimal: {})", result, result);
}
```

**Output:**

```
Left Shift: 00001000 (decimal: 8)
```

***

### 6. Right Shift (`>>`)

Shift bits right (divide by 2 each shift):

```rust
fn main() {
    let a: u8 = 0b1000; // 8

    let result = a >> 3;

    println!("Right Shift: {:08b} (decimal: {})", result, result);
}
```

**Output:**

```
Right Shift: 00000001 (decimal: 1)
```

***

### 7. Checking if a specific bit is set

```rust
fn main() {
    let number: u8 = 0b1010_0100;

    // Check if 5th bit (zero-based) is set
    let mask = 1 << 5;
    let is_set = (number & mask) != 0;

    println!("Is bit 5 set? {}", is_set);
}
```

**Output:**

```
Is bit 5 set? true
```

***

### 8. Toggle (flip) a bit

```rust
fn main() {
    let mut number: u8 = 0b0000_1111;

    // Toggle 3rd bit (zero-based)
    number ^= 1 << 3;

    println!("After toggle: {:08b}", number);
}
```

**Output:**

```
After toggle: 00000111
```

***

### Summary of Bitwise Operators in Rust

| Operator | Description           | Example   |
|----------|-----------------------|-----------|
| `&`      | Bitwise AND           | `a & b`   |
| `|`      | Bitwise OR            | `a | b`   |
| `^`      | Bitwise XOR           | `a ^ b`   |
| `!`      | Bitwise NOT           | `!a`      |
| `<<`     | Left shift            | `a << 2`  |
| `>>`     | Right shift           | `a >> 1`  |

***

These bitwise operations are very efficient for low-level manipulation of bits, useful in dynamic programming optimizations, flag handling, or encoding states compactly in Rust.[1][2][4]

[1](https://www.tutorialspoint.com/rust/rust_bitwise_operators.htm)
[2](https://rustic-chess.org/appendix/bitwise_operations.html)
[3](https://www.youtube.com/watch?v=tBk1jIQck5w)
[4](https://redandgreen.co.uk/bitwise-operators-in-rust/rust-programming/)
[5](https://togglebit.io/posts/rust-bitwise/)
[6](https://doc.rust-lang.org/book/appendix-02-operators.html)
[7](https://stackoverflow.com/questions/65136709/rust-bitwise-operations)
[8](https://www.reddit.com/r/rust/comments/1f3h738/bitwise_operations_on_booleans/)
[9](https://www.youtube.com/watch?v=6h89XQaGonE)
[10](https://www.reddit.com/r/programming/comments/13ejv0z/a_while_ago_i_created_a_web_app_to_help_me/)