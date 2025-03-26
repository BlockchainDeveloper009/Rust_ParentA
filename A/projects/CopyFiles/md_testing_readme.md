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



### Run Tests parallely

cargo test --test-threads 4

----------------

In Rust, there isn't a built-in way to categorize or tag tests the way Playwright or NUnit does. However, you can achieve similar functionality with **test attributes** and **custom filtering**.

Here’s how you can implement a tagging mechanism for your Rust tests and filter them when running with `cargo test`.

### 1. Tagging Tests Using Custom Attributes:
Rust doesn't have native support for custom tags, but you can use **attributes** to simulate this behavior.

One approach is to define a custom attribute and then use it as a tag, but Rust's test framework doesn't provide built-in support for this. Instead, you can organize your tests using `#[cfg]` attributes or custom `#[test]` attributes.

#### Example of using `#[cfg]` for test categorization:

Rust allows you to use `#[cfg(test)]` or `#[cfg(feature = "some_tag")]` to conditionally compile tests based on tags or features.

### 2. Example: Using `#[cfg]` with Tags

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "tag1")]
    fn test_tag1() {
        println!("This test is part of tag1");
    }

    #[test]
    #[cfg(feature = "tag2")]
    fn test_tag2() {
        println!("This test is part of tag2");
    }

    #[test]
    #[cfg(feature = "tag1")]
    fn test_tag1_again() {
        println!("This test is also part of tag1");
    }

    #[test]
    fn test_no_tag() {
        println!("This test has no tag");
    }
}
```

### 3. Running Specific Tests Based on Tags:

To filter the tests by tag, you can pass the `--features` flag to `cargo test`.

For example, to run only tests that are tagged with `tag1`:

```sh
cargo test --features tag1
```

Or for `tag2`:

```sh
cargo test --features tag2
```

### 4. Using Multiple Tags:

You can specify multiple tags (features) by separating them with commas:

```sh
cargo test --features tag1,tag2
```

This will run tests with both `tag1` and `tag2`.

### 5. Example of Using Custom Filtering with `#[cfg]`:

You can create a more granular filter mechanism by assigning custom tags to different test categories (e.g., `integration`, `unit`, `critical`).

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "unit")]
    fn test_unit() {
        println!("This is a unit test.");
    }

    #[test]
    #[cfg(feature = "integration")]
    fn test_integration() {
        println!("This is an integration test.");
    }

    #[test]
    #[cfg(feature = "critical")]
    fn test_critical() {
        println!("This is a critical test.");
    }
}
```

Then, run the tests by specifying the category you want to run:

```sh
cargo test --features unit
cargo test --features integration
cargo test --features critical
```

### 6. Filtering Tests Using `--test`:

If you don't want to use `#[cfg]` features and just want to filter by test name, you can run:

```sh
cargo test test_tag1
```

This will only run tests with `test_tag1` in the name.

### Conclusion:
While Rust doesn't have a built-in way to tag tests like Playwright or NUnit, you can use `#[cfg]` and features to simulate tagging and categorizing tests. This allows you to filter tests during runtime based on the tags you assign.

Let me know if you'd like further clarification or assistance with your test setup!