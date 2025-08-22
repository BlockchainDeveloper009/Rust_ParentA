Certainly! Here’s the same Rust code with detailed comments and explanations tailored for beginners or freshers:

***

### 1. Vector (Dynamic Array)
```rust
fn vec_example() {
    // Create an empty vector to store integers
    let mut numbers: Vec<i32> = Vec::new();

    // Add elements to the vector using push()
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    // Iterate over the vector and print each element
    for num in &numbers {
        println!("Number: {}", num);
    }
}
```
- **Vec<T>** is a growable array type in Rust.
- `Vec::new()` creates an empty vector.
- `.push(value)` adds an element to the end.
- `for num in &numbers` iterates over references to elements.

***

### 2. Multi-dimensional Vector (2D Matrix)
```rust
fn matrix_example() {
    // Create a 3x4 matrix initialized with zeros (3 rows, 4 columns)
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; 4]; 3];

    // Modify one element at row 1, column 2 (indexing starts at 0)
    matrix[1][1] = 42;

    // Print each element row by row
    for row in &matrix {
        for val in row {
            print!("{} ", val); // Print value with a space, no newline
        }
        println!(); // Newline after printing each row
    }
}
```
- `vec![value; n]` creates a vector with `n` copies of `value`.
- So `vec![vec![0; 4]; 3]` creates 3 rows, each with 4 zeros.
- Access elements using double indexing: `matrix[row][column]`.

***

### 3. Hash Map (Memoization)
```rust
use std::collections::HashMap;

fn hashmap_example() {
    // Create a HashMap to store key-value pairs, here keys are strings and values are integers
    let mut memo = HashMap::new();

    // Insert some key-value pairs
    memo.insert("apple", 3);
    memo.insert("banana", 2);

    // Retrieve a value associated with "apple"
    if let Some(&count) = memo.get("apple") {
        println!("Apple count: {}", count);
    }
}
```
- `HashMap<K, V>` allows storage of pairs: key -> value.
- Use `.insert(key, value)` to add entries.
- `.get(key)` returns an `Option<&V>` which you `unwrap` carefully.

***

### 4. Tuple (State Representation)
```rust
fn tuple_example() {
    // A tuple groups together multiple values, possibly of different types
    let state: (i32, i32) = (5, 10);

    // Access tuple elements via .0, .1, ...
    println!("State x: {}, y: {}", state.0, state.1);
}
```
- Tuples are fixed-size collections of heterogeneous types.
- Useful to represent a combined state compactly, for example `(row, column)`

***

### 5. Stack (using Vec)
```rust
fn stack_example() {
    // Vec can also be used as a stack (LIFO structure)
    let mut stack: Vec<i32> = Vec::new();

    // Push elements to the top of the stack
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // Pop elements from the top until stack is empty
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
}
```
- `.push()` adds items to the stack.
- `.pop()` removes the last added element (the "top") and returns it.
- `while let Some(item) = stack.pop()` keeps popping until empty.

***

### 6. Bitmask Example (using integers)
```rust
fn bitmask_example() {
    // Use bits in an integer to represent subsets or state flags
    let mask: u8 = 0b1010; // binary number representing bit pattern

    // Print the bit pattern using {:04b} (binary with 4 digits)
    println!("Bitmask: {:04b}", mask);

    // Check if the 2nd bit (from right, 0-based) is set (1)
    if mask & 0b0010 != 0 {
        println!("2nd bit is set");
    } else {
        println!("2nd bit is not set");
    }
}
```
- Bits in integers can represent compact flags or states.
- `&` is the bitwise AND operator used to test bits.
- Bitmasks are widely used to optimize DP when dealing with subsets.

***

### 7. Graph (Adjacency List)
```rust
fn graph_example() {
    // Represent graph as adjacency list:
    // Each node has a list of connected nodes (edges)
    let graph: Vec<Vec<usize>> = vec![
        vec![1, 2], // Node 0 connects to nodes 1 and 2
        vec![1],    // Node 1 connects to node 2
        vec![],     // Node 2 has no outgoing edges
    ];

    // Print edges from each node
    for (node, edges) in graph.iter().enumerate() {
        println!("Edges from node {}: {:?}", node, edges);
    }
}
```
- Graph stored as `Vec` of adjacency lists (`Vec<Vec<usize>>`).
- `graph[node]` returns the list of neighbors.

***

### 8. Priority Queue (Binary Heap)
```rust
use std::collections::BinaryHeap;

fn priority_queue_example() {
    // BinaryHeap stores elements ordered by priority (max at top)
    let mut heap = BinaryHeap::new();

    // Add some numbers
    heap.push(30);
    heap.push(10);
    heap.push(20);

    // Pop elements in order from largest to smallest
    while let Some(top) = heap.pop() {
        println!("Top value: {}", top);
    }
}
```
- `BinaryHeap<T>` lets you maintain a max-heap.
- `.push()` adds elements, `.pop()` removes the max element.
- Useful for DP optimization or best-first search algorithms.

***

If you want me to expand with example problems or how these fit in dynamic programming tasks, just ask!

[1](https://leetcode.ca/2015-12-03-3-Longest-Substring-Without-Repeating-Characters/)