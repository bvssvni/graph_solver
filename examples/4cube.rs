use graph_solver::*;

// Notice that edges starts with `2`.
const EDGE: Color = 2;

fn main() {
    let mut g = Graph::new();

    // Create a node pattern.
    let a = Node {
        color: 0,
        self_connected: false,
        edges: vec![Constraint {edge: EDGE, node: 0}; 4]
    };

    for _ in 0..16 {g.push(a.clone())}
    g.no_triangles = true;
    g.connected = true;

    let entropy_solve_settings = EntropySolveSettings::new()
        .attempts(1000)
        .noise(0.5)
        .final_attempt(Some(None));
    let solve_settings = SolveSettings::new(); // .debug(true).sleep_ms(10);
    if let (_n, Some(solution)) = g.solve(entropy_solve_settings, solve_settings) {
        // println!("Attempts: {}", n);
        // solution.puzzle.print();
        println!("{}", solution.puzzle.graphviz(
            "sfdp",
            &["black,fontcolor=white"],
            &["black"]
        ));
    } else {
        eprintln!("<no solution>");
    }
}
