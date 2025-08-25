DFS.md
Here's a basic implementation of Depth First Search (DFS) in Rust using an adjacency list representation for an undirected graph:

```rust
use std::collections::HashSet;

struct Graph {
    adj_list: Vec<Vec<usize>>,
}

impl Graph {
    fn new(num_nodes: usize) -> Self {
        Graph {
            adj_list: vec![vec![]; num_nodes],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_list[u].push(v);
        self.adj_list[v].push(u);
    }

    fn dfs(&self, start: usize, visited: &mut HashSet<usize>) {
        visited.insert(start);
        println!("Visited node {}", start);
        for &neighbor in &self.adj_list[start] {
            if !visited.contains(&neighbor) {
                self.dfs(neighbor, visited);
            }
        }
    }
}

fn main() {
    let mut graph = Graph::new(5);

    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(1, 4);

    let mut visited = HashSet::new();
    graph.dfs(0, &mut visited);
}
```

***

### Explanation:
- **Graph structure** uses a `Vec<Vec<usize>>` where each vector holds neighbors of a node.
- `add_edge` adds an undirected edge by pushing each node to the other's adjacency list.
- `dfs` function recursively visits nodes, marking them as visited using the `HashSet`.
- The main function constructs the graph and performs DFS starting from node 0.

***

### Sample Output:

```
Visited node 0
Visited node 1
Visited node 3
Visited node 4
Visited node 2
```

***

This simple example demonstrates how to perform a standard recursive DFS in Rust. It’s suitable for interview usage and can be extended for directed graphs or more complex traversals. Let me know if you want a non-recursive DFS or more advanced features!

[1](https://www.youtube.com/watch?v=hQKFaD7e564)
[2](https://www.programiz.com/dsa/graph-dfs)
[3](https://shadow.github.io/docs/rust/petgraph/visit/fn.depth_first_search.html)
[4](https://stackoverflow.com/questions/73270572/implementing-depth-first-search-using-a-stack)
[5](https://github.com/TheAlgorithms/Rust/blob/master/src/graph/depth_first_search.rs)
[6](https://www.reddit.com/r/rust/comments/1gu3mwj/i_built_easytree_a_rust_library_for_tree/)
[7](https://docs.rs/petgraph/latest/petgraph/visit/struct.Dfs.html)
[8](https://docs.rs/pathfinding/latest/pathfinding/?search=dfs)
[9](https://depth-first.com/articles/2020/02/03/graphs-in-rust-an-introduction-to-petgraph/)