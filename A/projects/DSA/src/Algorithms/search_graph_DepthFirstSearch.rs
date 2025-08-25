use std::collections::HashSet;

// Define a simple undirected graph using adjacency lists
struct Graph {
    // Each node's neighbors are stored in a vector of usize indices
    adj_list: Vec<Vec<usize>>,
}

impl Graph {
    // Create a new graph with `num_nodes` nodes, each with an empty adjacency list
    fn new(num_nodes: usize) -> Self {
        Graph {
            adj_list: vec![vec![]; num_nodes], // Initialize adjacency vector for each node
        }
    }

    // Add an undirected edge between nodes `u` and `v`
    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_list[u].push(v); // Add v to u's neighbors
        self.adj_list[v].push(u); // Add u to v's neighbors (undirected graph)
    }

    // Perform Depth First Search starting from node `start`
    // `visited` keeps track of visited nodes to avoid cycles and repeated visits
    fn dfs(&self, start: usize, visited: &mut HashSet<usize>) {
        visited.insert(start); // Mark the current node as visited
        println!("Visited node {}", start); // Process the current node (here, just print)
        // Recursively visit all neighbors that are not yet visited
        for &neighbor in &self.adj_list[start] {
            if !visited.contains(&neighbor) {
                self.dfs(neighbor, visited); // Recursive call on neighbor
            }
        }
    }
}

fn main() {
    let mut graph = Graph::new(5); // Create a graph with 5 nodes (0 through 4)

    // Add edges between nodes (undirected)
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(1, 4);

    let mut visited = HashSet::new(); // Track visited nodes
    graph.dfs(0, &mut visited); // Start DFS from node 0
}
