use graph_solver::*;

// Notice that edges starts with `2`.
const RED: Color = 2;

fn main() {
    let mut g = Graph::new();
    let a = Node {
        color: 0,
        self_connected: false,
        edges: vec![Constraint {edge: RED, node: 1}]
    };
    let b = Node {
        color: 1,
        self_connected: false,
        edges: vec![Constraint {edge: RED, node: 0}]
    };
    g.push(a);
    g.push(b);

    let entropy_solve_settings = EntropySolveSettings::new()
        .attempts(1000)
        .noise(0.5)
        .final_attempt(Some(None));
    let solve_settings = SolveSettings::new();
    if let (_n, Some(solution)) = g.solve(entropy_solve_settings, solve_settings) {
        solution.puzzle.print();
    }
}
