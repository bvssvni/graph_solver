/*
=== CUBE EXAMPLE ===

Run with GraphViz (https://graphviz.org/):

    cargo run --example cube | dot -Tpng > test.png

*/

use graph_solver::*;

// Notice that edges starts with `2`.
// This is because `0` is empty and `1` is no-edge.
const EDGE: Color = 2;

fn main() {
    let mut g = Graph::new();

    // Create a node pattern with 3 edges.
    let a = Node {
        color: 0,
        self_connected: false,
        edges: vec![Constraint {edge: EDGE, node: 0}; 3]
    };

    // Add 8 vertices.
    for _ in 0..8 {g.push(a.clone())}
    g.no_triangles = true;

    let entropy_solve_settings = EntropySolveSettings::new()
        .attempts(1000)
        .noise(0.5)
        .final_attempt(Some(None));
    let solve_settings = SolveSettings::new();
    if let (_n, Some(solution)) = g.solve(entropy_solve_settings, solve_settings) {
        println!("{}", solution.puzzle.graphviz(
            "sfdp",
            &["black"],
            &["black"]
        ));
    } else {
        eprintln!("<no solution>");
    }
}
