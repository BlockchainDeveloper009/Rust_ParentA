BFS.md
Here's the Breadth-First Search (BFS) implementation in Rust with detailed comments and explanations:

```rust
use std::collections::{HashSet, VecDeque};

// Define a graph structure using adjacency list representation
struct Graph {
    adj_list: Vec<Vec<usize>>, // adjacency list storing neighbors for each node
}

impl Graph {
    // Initialize a graph with `num_nodes` nodes, each with an empty list of neighbors
    fn new(num_nodes: usize) -> Self {
        Graph {
            adj_list: vec![vec![]; num_nodes], // create empty adjacency list for each node
        }
    }

    // Add an undirected edge between two nodes u and v
    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_list[u].push(v); // add v to u's neighbors
        self.adj_list[v].push(u); // add u to v's neighbors (undirected graph)
    }

    // Perform BFS starting from the `start` node
    // Returns a vector of nodes in the order they are visited
    fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new(); // Track visited nodes
        let mut queue = VecDeque::new(); // Use queue for BFS: FIFO

        let mut order = Vec::new(); // To record BFS traversal order

        visited.insert(start);
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            order.push(node);
            // Explore all neighbors of current node
            for &neighbor in &self.adj_list[node] {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
        order
    }
}

fn main() {
    let mut graph = Graph::new(6);

    // Add some undirected edges
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(1, 4);
    graph.add_edge(2, 4);
    graph.add_edge(3, 5);
    graph.add_edge(4, 5);

    // Execute BFS from node 0
    let bfs_order = graph.bfs(0);

    println!("BFS traversal order: {:?}", bfs_order);
}
```

***

### Explanation:

- **Graph Representation:** Uses an adjacency list where each node stores its neighbors in a vector.
- **Adding Edges:** The `add_edge` function adds edges bidirectionally because it's an undirected graph.
- **Visited Set:** A hash set keeps track of which nodes have been visited to prevent revisiting and infinite loops.
- **Queue for BFS:** `VecDeque` is used as a queue to process nodes in FIFO order.
- **BFS Algorithm:**  
  - Start with the `start` node, mark it visited, and enqueue it.  
  - While the queue isn’t empty, dequeue a node, process it (add to order), then enqueue all its unvisited neighbors.  
- **Traversal Order:** Returned as a vector showing the order nodes are visited.

***

### Sample Output:

```
BFS traversal order: [0, 1, 2, 3, 4, 5]
```

***

This BFS code with comments is suitable for interview preparation and clearly highlights the key concepts involved in the traversal. Let me know if you want a non-recursive DFS or weighted graph BFS!

[1](https://dev.to/wrongbyte/introduction-to-graphs-bfs-algorithm-mld)
[2](https://www.linkedin.com/pulse/efficient-bfs-rust-christian-pflugradt-asm0e)
[3](https://www.youtube.com/watch?v=ZDy3tqn-DKA)
[4](https://stackoverflow.com/questions/73553178/breadth-first-traversal-of-binary-tree-in-rust)
[5](https://dev.to/fushji/a-weekly-rust-pill-5-5d5j)
[6](https://news.ycombinator.com/item?id=17592375)
[7](https://www.reddit.com/r/learnrust/comments/1089is3/cant_solve_an_ownership_issue_while_implementing/)
[8](https://gist.github.com/vTurbine/16fbb99225ad4c0ac80b24855dd61a7c)