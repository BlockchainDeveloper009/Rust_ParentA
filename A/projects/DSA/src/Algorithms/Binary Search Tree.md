Binary Search Tree.md


Here's a simple, clear implementation of a Binary Search Tree (BST) in Rust with detailed comments and explanations:

```rust
use std::cmp::Ordering;

// Node of the Binary Search Tree
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,  // Left child
    right: Option<Box<Node<T>>>, // Right child
}

// BST structure containing the root node
pub struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
    // Create a new empty BST
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a new value into BST
    pub fn insert(&mut self, value: T) {
        self.root = Self::insert_node(self.root.take(), value);
    }

    // Helper recursive function to insert a node
    fn insert_node(node: Option<Box<Node<T>>>, value: T) -> Option<Box<Node<T>>> {
        match node {
            None => {
                // If spot is empty, put the new node here
                Some(Box::new(Node { value, left: None, right: None }))
            }
            Some(mut current_node) => {
                // Decide to insert left or right recursively
                match value.cmp(&current_node.value) {
                    Ordering::Less => {
                        current_node.left = Self::insert_node(current_node.left.take(), value);
                    }
                    Ordering::Greater => {
                        current_node.right = Self::insert_node(current_node.right.take(), value);
                    }
                    Ordering::Equal => {
                        // Duplicate values are not inserted
                    }
                }
                Some(current_node)
            }
        }
    }

    // Search for a value in BST, return true if found
    pub fn search(&self, value: &T) -> bool {
        Self::search_node(&self.root, value)
    }

    // Helper recursive function to search value
    fn search_node(node: &Option<Box<Node<T>>>, value: &T) -> bool {
        match node {
            None => false, // Not found
            Some(current_node) => match value.cmp(&current_node.value) {
                Ordering::Equal => true, // Found
                Ordering::Less => Self::search_node(&current_node.left, value),
                Ordering::Greater => Self::search_node(&current_node.right, value),
            },
        }
    }
}

fn main() {
    let mut bst = BinarySearchTree::new();

    bst.insert(10);
    bst.insert(5);
    bst.insert(15);
    bst.insert(3);
    bst.insert(7);

    println!("Search 7: {}", bst.search(&7));  // true
    println!("Search 8: {}", bst.search(&8));  // false
}
```

***

### Explanation:

- **Node Struct:** Contains a value and optional boxed left and right child nodes.
- **BinarySearchTree Struct:** Holds the root node (or None if empty).
- **Insert:** Uses recursion to find the correct leaf spot. If the spot is empty, inserts a new node.
- **Search:** Recursively compares the target value with current node, moving left or right accordingly.
- **Box:** Used to enable recursive types with heap allocation.
- **Option:** To represent the presence or absence of children.

***

### Example Output:

```
Search 7: true
Search 8: false
```

***

This implementation demonstrates basic BST operations in safe Rust with clear ownership and borrowing. It can be extended with delete, traversal, balancing, and more features as needed. Let me know if you want those as well!

[1](https://github.com/TheAlgorithms/Rust/blob/master/src/data_structures/binary_search_tree.rs)
[2](https://github.com/starovoid/binary_search_tree)
[3](https://stackoverflow.com/questions/74371597/implement-the-recursive-insert-method-of-binary-search-tree-in-rust)
[4](https://docs.rs/bst-rs)
[5](https://users.rust-lang.org/t/binary-search-tree-insert-function-with-only-safe-rust/80681)
[6](https://www.reddit.com/r/rust/comments/s5bxt4/binary_search_tree_rust/)
[7](https://www.youtube.com/watch?v=yHi3q2Iiepc)
[8](https://users.rust-lang.org/t/simplify-tree-implementation/85597)