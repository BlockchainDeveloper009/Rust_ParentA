ds.md
give me sample code for creating, populating and using them 

Here are simple Rust code examples demonstrating how to create, populate, and use the key data structures useful for dynamic programming:

***

### 1. Vector (Dynamic Array)
```rust
fn vec_example() {
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    for num in &numbers {
        println!("Number: {}", num);
    }
}
```

***

### 2. Multi-dimensional Vector (2D Matrix)
```rust
fn matrix_example() {
    // Create 3x4 matrix filled with zeros
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; 4]; 3];

    matrix[1][10] = 42;

    for row in &matrix {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }
}
```

***

### 3. Hash Map (Memoization)
```rust
use std::collections::HashMap;

fn hashmap_example() {
    let mut memo = HashMap::new();

    memo.insert("apple", 3);
    memo.insert("banana", 2);

    if let Some(&count) = memo.get("apple") {
        println!("Apple count: {}", count);
    }
}
```

***

### 4. Tuple (State Representation)
```rust
fn tuple_example() {
    let state: (i32, i32) = (5, 10);
    println!("State x: {}, y: {}", state.0, state.1);
}
```

***

### 5. Stack (using Vec)
```rust
fn stack_example() {
    let mut stack: Vec<i32> = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
}
```

***

### 6. Bitmask Example (using integers)
```rust
fn bitmask_example() {
    let mask: u8 = 0b1010; // binary representation
    println!("Bitmask: {:04b}", mask);

    // Test if 2nd bit is set (counting from right, zero-based)
    if mask & 0b0010 != 0 {
        println!("2nd bit is set");
    } else {
        println!("2nd bit is not set");
    }
}
```

***

### 7. Graph (Adjacency List)
```rust
fn graph_example() {
    // Directed graph adjacency list for 3 nodes (0, 1, 2)
    let graph: Vec<Vec<usize>> = vec![
        vec![1, 2], // Node 0 -> 1, 2
        vec![2],    // Node 1 -> 2
        vec![],     // Node 2 -> no edges
    ];

    for (node, edges) in graph.iter().enumerate() {
        println!("Edges from node {}: {:?}", node, edges);
    }
}
```

***

### 8. Priority Queue (Binary Heap)
```rust
use std::collections::BinaryHeap;

fn priority_queue_example() {
    let mut heap = BinaryHeap::new();

    heap.push(30);
    heap.push(10);
    heap.push(20);

    while let Some(top) = heap.pop() {
        println!("Top value: {}", top);
    }
}
```

***

These examples should give you practical starting points for using the most important data structures commonly used in dynamic programming with Rust. Let me know if you'd like any example expanded or explained step-by-step!

[1](https://leapcell.io/blog/rust-data-structures-guide)
[2](https://www.reddit.com/r/rust/comments/ceexl0/how_to_create_dynamic_data_structures_in_rust/)
[3](https://dsar.rantai.dev/docs/part-i/chapter-2/)
[4](https://users.rust-lang.org/t/how-does-rust-handle-dynamic-data-types/95705)
[5](https://users.rust-lang.org/t/dynamic-data-structure/91422)
[6](https://www.youtube.com/watch?v=MV-Zca7Unxo)
[7](https://www.youtube.com/watch?v=fm80-oJGTUw)
[8](https://users.rust-lang.org/t/which-of-the-two-data-structure-is-more-efficient/79134)
[9](https://github.com/alexfertel/rust-algorithms)
[10](https://leetcode.ca/2015-12-03-3-Longest-Substring-Without-Repeating-Characters/)