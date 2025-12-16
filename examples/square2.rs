use graph_solver::*;

// Notice that edges starts with `2`.
const SOLID: Color = 2;

fn main() {
    let mut g = Graph::new();

    // Create a node pattern.
    let a = Node {
        color: 0,
        self_connected: false,
        edges: vec![Constraint {edge: SOLID, node: 0}; 2]
    };

    // Add 4 nodes.
    for _ in 0..4 {g.push(a.clone())}

    let entropy_solve_settings = EntropySolveSettings::new()
        .attempts(1000)
        .noise(0.5)
        .final_attempt(Some(None));
    let solve_settings = SolveSettings::new()
        .debug(true).sleep_ms(2000);
    if let (_n, Some(solution)) = g.solve(entropy_solve_settings, solve_settings) {
        // Prints:
        // 0 0 0 0
        // ========================================
        // 0 2 1 0
        // 2 0 0 1
        // 1 0 0 2
        // 0 1 2 0
        solution.puzzle.print();
    }
}
