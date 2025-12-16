use graph_solver::*;

const RED: Color = 2;
const GREEN: Color = 3;
const GREEN_DASHED: Color = 4;

fn main() {
    let mut g = Graph::new();
    let a = Node {
        color: 0,
        self_connected: false,
        edges: vec![
            Constraint {edge: RED, node: 1},
            Constraint {edge: GREEN, node: 1},
        ]
    };
    let b = Node {
        color: 1,
        self_connected: false,
        edges: vec![
            Constraint {edge: RED, node: 0},
            Constraint {edge: GREEN_DASHED, node: 0},
        ]
    };
    let c = Node {
        color: 0,
        self_connected: false,
        edges: vec![
            Constraint {edge: RED, node: 1},
            Constraint {edge: GREEN_DASHED, node: 1},
        ]
    };
    let d = Node {
        color: 1,
        self_connected: false,
        edges: vec![
            Constraint {edge: RED, node: 0},
            Constraint {edge: GREEN, node: 0},
        ]
    };
    g.push(a);
    g.push(b);
    g.push(c);
    g.push(d);

    let entropy_solve_settings = EntropySolveSettings::new()
        .attempts(1000)
        .noise(0.5)
        .final_attempt(Some(None));
    let solve_settings = SolveSettings::new();
    if let (_n, Some(solution)) = g.solve(entropy_solve_settings, solve_settings) {
        // solution.puzzle.print();
        let nodes = &["black", "white"];
        let edges = &["red", "green", "green,style=dashed"];
        println!("{}", solution.puzzle.graphviz("sfdp", nodes, edges));
    }
}
