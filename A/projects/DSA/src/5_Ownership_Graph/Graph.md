Graph.md
MAAGA companies (Microsoft, Apple, Amazon, Google, Airbnb) typically **test ownership and borrowing in Rust interviews with graph problems** by focusing on how candidates handle mutable and immutable references, movement of resources, and lifetime management within complex structures like graphs.

Here’s how these concepts commonly come up in interviews:

***

## 1. **Ownership Concepts in Graphs:**

- **Node and Edge Storage**: You might be asked to design a graph (adjacency list or adjacency matrix) with `Vec`, `HashMap`, or custom structs.
- **Moving Ownership**: Passing ownership of nodes or edges between functions or structs, making sure there's only one owner at a time unless data is `Copy`.
- **Drop and Lifetime**: You may be asked what happens when graph objects go out of scope, or to explain lifetimes if nodes have references to other nodes.

**Example**:  
Design a graph structure where you add and remove nodes. Does your code maintain memory safety? Does a node get invalidated if ownership is moved?


## 2. **Borrowing Concepts in Graphs:**

- **Mutable vs Immutable References**: You'll need to traverse and maybe update nodes and edges, being careful with how you borrow data.
- **Borrow Checker**: Rust forces you to use either multiple immutable references or a single mutable reference. You may need to split a graph traversal (DFS/BFS) into separate phases or refactor code to satisfy the borrow checker.

**Example**:  
Write a function that marks all reachable nodes from a starting node, but the marking must not violate the borrow checker rules on mutable and immutable access.


## 3. **Typical Interview Tasks to Test These Concepts:**
- **Implement DFS or BFS**: Traverse a graph without running afoul of the borrow checker.
- **Cycle Detection (DFS)**: Show mutability and borrowing within recursive visits.
- **Topological Sort**: Manage visited flags and adjacency lists safely.
- **Graph Copying/Cloning**: Show how you handle ownership when copying parts of a graph or borrowing for read-only traversal.
- **Edge Cases**: Interviewers may ask what happens if you try to borrow mutably and immutably at the same time, or about lifetime annotations on graph nodes.

***

## 4. **Common Pitfalls Interviewers Look For:**
- Using two mutable references to the same object (compile fail).
- Dangling references after moving a node.
- Not releasing borrows before making a conflicting borrow.
- Lifetime misunderstanding when building graph structures with reference cycles.

***

## 5. **Sample Rust Borrowing in Graph (Pseudocode):**
```rust
struct Node {
    value: i32,
    neighbors: Vec<usize>
}

struct Graph {
    nodes: Vec<Node>
}

impl Graph {
    fn get_value(&self, idx: usize) -> &i32 {
        &self.nodes[idx].value
    }

    fn add_value(&mut self, idx: usize, val: i32) {
        self.nodes[idx].value = val;
    }
}
```
Here, note difference between `&self` for immutable borrowing (reading) and `&mut self` for mutable borrowing (writing).

***

## Resources for Visualization and Study:
- **RustViz and tools** visually demonstrate ownership/borrowing events alongside Rust graph code for learning and interview prep.[1][2]
- More on [borrow checker and references in Rust].[3]

***

**In summary:**  
You’ll be tested on your ability to structure graph data safely, manage mutable and immutable accesses correctly, and demonstrate lifetime understanding—all while solving realistic graph problems in Rust’s unique ownership/borrowing model. Practice both coding and explaining your reasoning for borrow checker decisions!

[1](https://web.eecs.umich.edu/~comar/rustviz-vlhcc22.pdf)
[2](https://arxiv.org/pdf/2011.09012.pdf)
[3](https://www.integralist.co.uk/posts/rust-ownership/)
[4](https://www.reddit.com/r/rust/comments/vkbeuk/why_the_ownershipborrowing_model/)
[5](https://metana.io/blog/rust-ownership-and-borrowing-simplified/)
[6](https://www.geeksforgeeks.org/dsa/top-50-graph-coding-problems-for-interviews/)
[7](https://www.youtube.com/watch?v=79phqVpE7cU)
[8](https://bcls.lib.nj.us)
[9](https://cuyahogalibrary.org)