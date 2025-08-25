MinimumSpanningTree.md

Here's a simple Rust implementation of Kruskal's algorithm to find the Minimum Spanning Tree (MST) of a connected, undirected graph:

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Edge {
    u: usize,
    v: usize,
    weight: i32,
}

// Disjoint Set Union (Union-Find) data structure to manage connected components efficiently
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false; // Already connected
        }
        // Union by rank
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
        true
    }
}

fn kruskal_mst(num_nodes: usize, mut edges: Vec<Edge>) -> Vec<Edge> {
    // Sort edges by weight
    edges.sort_by_key(|e| e.weight);
    let mut uf = UnionFind::new(num_nodes);
    let mut mst = Vec::new();

    for edge in edges {
        if uf.union(edge.u, edge.v) {
            mst.push(edge);
        }
    }
    mst
}

fn main() {
    let num_nodes = 5;
    let edges = vec![
        Edge { u:0, v:1, weight:10 },
        Edge { u:0, v:2, weight:6 },
        Edge { u:0, v:3, weight:5 },
        Edge { u:1, v:3, weight:15 },
        Edge { u:2, v:3, weight:4 },
    ];

    let mst = kruskal_mst(num_nodes, edges);

    println!("Edges in MST:");
    let mut total_weight = 0;
    for edge in &mst {
        println!("{} -- {} == {}", edge.u, edge.v, edge.weight);
        total_weight += edge.weight;
    }
    println!("Total weight of MST: {}", total_weight);
}
```

***

### Explanation:
- The graph has nodes indexed from `0` to `num_nodes - 1`.
- `Edge` struct stores endpoints and weights.
- `UnionFind` tracks connected components to avoid cycles.
- Kruskal's algorithm sorts edges by weight and picks edges that don't form cycles.
- The returned vector `mst` contains the edges in the MST.
- You can run `cargo run` on this code to see the MST edges and total weight.

***

### Sample Output:

```
Edges in MST:
2 -- 3 == 4
0 -- 3 == 5
0 -- 1 == 10
Total weight of MST: 19
```

***

This example gives you a basic, efficient MST implementation in Rust with typical interview-friendly data structures and idiomatic Rust techniques. Let me know if you want Prim’s algorithm or a weighted graph using adjacency lists!

[1](https://www.rustworkx.org/apiref/rustworkx.minimum_spanning_tree.html)
[2](https://www.rustworkx.org/apiref/rustworkx.minimum_spanning_edges.html)
[3](https://www.reddit.com/r/learnprogramming/comments/12kys3m/finding_all_minimum_spanning_trees_without_brute/)
[4](https://shadow.github.io/docs/rust/petgraph/algo/min_spanning_tree/fn.min_spanning_tree.html)
[5](https://docs.rs/nopetgraph/latest/petgraph/algo/fn.min_spanning_tree.html)
[6](https://sampleprograms.io/projects/minimum-spanning-tree/rust/)
[7](https://users.rust-lang.org/t/implementing-distributed-minimum-spanning-tree-in-rust/40764)
[8](https://www.youtube.com/watch?v=gd7MjEbG8Rw)
[9](https://stackoverflow.com/questions/2935754/all-minimum-spanning-trees-implementation)