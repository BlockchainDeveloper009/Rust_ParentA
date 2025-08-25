Branch-and-bound and dynamic programming (DP) are both exact algorithms for 0/1 knapsack, but their practical runtime characteristics differ depending on instance size and structure:

***

### **Dynamic Programming Runtime**

- **Complexity:** $$O(nW)$$ time and space, where $$n$$ = items and $$W$$ = knapsack capacity.
- **Performance:** Very efficient for small to medium $$n\times W$$. Scales well if $$W$$ isn’t huge.
- **Limitations:** Performance and memory get problematic as either $$n$$ or $$W$$ grow large, especially for large weights/capacities.[1][2]
- **Predictability:** Runtime is predictable for all problem shapes (each subproblem solved once).

***

### **Branch-and-Bound Runtime**

- **Complexity:** Worst case is exponential ($$O(2^n)$$), but clever pruning usually yields much better performance in practice.
- **Performance:** In "easy" instances (where many branches can be pruned due to bounds or infeasibility), branch-and-bound requires exploring far fewer subproblems than brute-force.
- **Limitations:** For “hard” instances with few pruned branches, can degrade towards brute-force performance.
- **Space:** Can also use less space than DP.
- **Predictability:** Runtime is problem-dependent and less predictable; highly efficient on some instances, slower on others.[2][3][4][1]

***

### **Practical Comparison**

- **Small/Medium Instances:** DP is usually fastest and most reliable, with guaranteed runtime.
- **Large $$W$$:** If the knapsack capacity $$W$$ is very large (out of memory bounds), or weights are not integers, DP may be impractical—branch-and-bound is preferred.[1][2]
- **Structured/Sparse Instances:** If many items or capacities can be easily pruned by bounding (e.g., most subsets are infeasible), branch-and-bound is often much faster than DP.[3][2][1]
- **Very Large $$n$$:** Both methods slow down, but branch-and-bound may handle large $$n$$ better if pruning is effective.

> "DP is more efficient for small to medium-sized instances [...] in general, dynamic programming provides a more effective solution. Branch-and-bound can be more efficient if the problem allows much pruning, and it is preferred when DP’s space or time are prohibitive."[2][1]

***

### **Summary Table**

| Algorithm         | Worst-Case Runtime      | Typical Use              | Space Usage      | Predictability     |
|-------------------|------------------------|--------------------------|------------------|-------------------|
| Dynamic Programming | $$O(nW)$$               | Small/medium, dense/structured | High (O(nW))    | Highly predictable|
| Branch-and-Bound | $$O(2^n)$$, often better | Large, sparse/prunable   | Low to moderate  | Problem dependent |

***

**In practice**:  
- Use DP for small-mid $$n,W$$ or when you need guaranteed performance.
- Try branch-and-bound for large instances, or when you expect effective pruning.

You may want to actually benchmark both for your problem shape, as the crossover point varies by input structure and hardware.[3][1][2]

[1](https://stackoverflow.com/questions/17205016/when-to-switch-from-dynamic-programming-2d-table-to-branch-bound-algorithm)
[2](http://paper.ijcsns.org/07_book/202402/20240202.pdf)
[3](https://www.geeksforgeeks.org/dsa/0-1-knapsack-using-branch-and-bound/)
[4](https://www.cs.emory.edu/~cheung/Courses/253/Syllabus/BranchBound/Docs/Knapsack-branch+bound.pdf)
[5](https://blogs.asarkar.com/assets/docs/algorithms/Knapsack%20-%20Shaheen+Sleit.pdf)
[6](https://www.sciencedirect.com/science/article/abs/pii/S030505481400063X)
[7](https://en.wikipedia.org/wiki/Knapsack_problem)