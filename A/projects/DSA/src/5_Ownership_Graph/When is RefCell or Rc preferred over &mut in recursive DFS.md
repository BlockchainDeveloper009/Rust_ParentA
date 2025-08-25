`RefCell` and `Rc` are **preferred over `&mut` in recursive DFS** when you need to:

### 1. **Mutate Data When You Don't Have Exclusive Ownership**

- With recursion and graph traversals, you often want to mutate nodes or their metadata, but you only have a shared (immutable) reference to the overall structure, or multiple references to the same node exist.
- `RefCell<T>` allows you to use **interior mutability**—you can mutate the contents even if you have an immutable reference to the `RefCell`.
- The borrow checker’s restrictions are checked **at runtime** instead of compile time, so your code can compile even in patterns where `&mut` wouldn’t be possible.

### 2. **Have Multiple Owners (Shared References)**

- In cyclic graphs, or when nodes can be referenced by multiple other nodes (parents, neighbors), you need **shared ownership**. 
- Rust’s `Rc<T>` (Reference Counting) allows many parts of your code to “own” a node.
- You combine `Rc<RefCell<T>>` to allow multiple owners who can also mutate the data if needed.

**Typical Example:**
```rust
use std::cell::RefCell;
use std::rc::Rc;

type NodeRef = Rc<RefCell<Node>>;

struct Node {
    value: i32,
    neighbors: Vec<NodeRef>,
}

fn dfs(node: &NodeRef) {
    // Mutate node value (interior mutable)
    node.borrow_mut().value += 1;
    for neighbor in node.borrow().neighbors.iter() {
        dfs(neighbor);
    }
}
```
Here, each node is shared (Rc) and mutable (RefCell), so you can both mutate and recursively pass references.

### 3. **When Is `&mut` Sufficient or Better?**

- If you have **exclusive ownership** of all nodes (like if using indices, or your graph structure doesn’t have cycles/shared references), and you can pass mutable references, prefer `&mut`! It’s safer and errors are caught at compile time.
- If you try to do recursive mutation with `&mut` but have cycles or multiple parents, the borrow checker will reject your code. In these cases, you need `Rc<RefCell<T>>`.

### 4. **Tradeoffs**

- **`RefCell` can panic at runtime** if you attempt to borrow mutably while there’s a live immutable borrow (violates borrow rules).
- **`Rc` is not thread-safe**; for multi-threading, use `Arc`.
- Relying on runtime borrowing (`RefCell`) can hide bugs until execution, but sometimes it’s necessary for graph structures.

### Summary Table

| Scenario                                          | Preference           |
|---------------------------------------------------|----------------------|
| Exclusive ownership, tree, simple graph           | `&mut`               |
| Cyclic graph, shared owners, parents/neighbors    | `Rc<RefCell<T>>`     |
| Need to mutate through shared reference, NOT `&mut`| `RefCell`            |

***

**In interviews, use `Rc<RefCell<T>>` when:**
- Nodes have multiple parents/neighbors.
- Mutation from multiple places is needed.
- The borrow checker cannot be satisfied with `&mut`.

But **prefer `&mut` where possible** for compile-time safety.[1][2][3][4][5][8]

[1](https://users.rust-lang.org/t/whats-difference-between-refcell-rc-t-and-rc-refcell-t/86497)
[2](https://stackoverflow.com/questions/57367092/what-is-the-difference-between-rcrefcellt-and-refcellrct/57367267)
[3](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch15-05-interior-mutability.html)
[4](https://www.reddit.com/r/rust/comments/llzewm/when_should_i_use_box_arc_rc_cell_and_refcell_can/)
[5](https://news.ycombinator.com/item?id=36342698)
[6](https://stackoverflow.com/questions/30275982/when-i-can-use-either-cell-or-refcell-which-should-i-choose)
[7](https://users.rust-lang.org/t/recursive-iterator-with-refcell-data-owned-by-the-current-function/91887)
[8](https://dsar.rantai.dev/docs/part-iii/chapter-15/)
[9](https://ftp.riken.jp/Linux/opensuse/distribution/leap/15.6/ChangeLogs/ChangeLog.openSUSE-Leap-15.6-x86_64-aarch64-ppc64le-s390x-Build568.1-Media1.txt)