Run Integration tests

cargo test --test handle_files_test


Run Unit tests:

cargo test


Type of Test	Location	Purpose
Unit Tests	Inside src/ in the same module	Test individual functions, have access to private code
Integration Tests	Outside src/ in tests/	Test public APIs and module interactions
Placing tests outside src/ ensures they only interact with the public API, preventing direct access to internal details.

Let me know if you need more clarification! 🚀


To **run a single test method** from a test file in Rust, use the following command:

### **Run a Specific Test from a File**
```sh
cargo test test_function_name
```
Replace `test_function_name` with the exact name of the test you want to run.

---

### **Example**
📌 **Test File: `tests/my_test.rs`**
```rust
#[test]
fn test_addition() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_subtraction() {
    assert_eq!(5 - 3, 2);
}
```

📌 **Run only `test_addition`:**
```sh
cargo test test_addition
```

---

### **Run a Specific Test from a Specific File**
If you want to run a test from a specific integration test file (in `tests/` directory), use:
```sh
cargo test --test my_test test_addition
```
Here:
- `--test my_test` runs the `tests/my_test.rs` file.
- `test_addition` ensures only `test_addition()` is executed.

---

### **Running Tests with Partial Names**
Rust allows partial test name matching:
```sh
cargo test addition
```
This will match any test that contains `"addition"` in its name.

---

Let me know if you need further clarification! 🚀

