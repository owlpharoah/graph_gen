# graph_builder

A simple Rust program that builds a graph from a list of vertices and connects them based on a specified degree. Useful for generating test graphs or understanding how uniform-degree graphs can be constructed.

---

## What it does

- Creates a graph with `n` vertices labeled from 1 to `n`
- Connects each vertex to the next `d` vertices (wrapping around) to generate uniform degree
- Stores edges in a `HashSet<(i32, i32)>` to avoid duplicates and ensure undirected connections

---

## Example

```rust
let mut g = Graph::new();
g.set_vertices(4);
let edges = make_edges(2, &mut g);
```

This produces a graph like:

```
Vertices: [1, 2, 3, 4]
Edges: {(1, 2), (1, 3), (2, 3), (2, 4), (3, 4), (1, 4)}
```

---

## API overview

```rust
struct Graph {
    vertices: Vec<i32>,
    edges: HashSet<(i32, i32)>,
}
```

### Methods

- `Graph::new()` – create an empty graph
- `set_vertices(n)` – add vertices from 1 to `n`
- `make_edges(d, &mut graph)` – connect each vertex to next `d` nodes
- Accessors:
  - `return_edges()`
  - `return_vertices()`
  - `return_n()` – total vertices

---

## Input validation

- The program checks if the given degree `d` is valid (i.e., `d <= n - 1`)
- If not, it prints an error and exits cleanly

---

## Run it

```bash
cargo run
```

The output will be a printed list of edges if a valid graph was generated.

---

## In progress

Currently extending this to support:

- Bipartite graphs

---

