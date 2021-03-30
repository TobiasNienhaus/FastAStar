mod f64_extensions;
mod weight;

use f64_extensions as r;

fn main() {
    let mut v: Vec<f64> = (1..100).map(|_| r::random_f64_inf_threshold(0.3)).collect();
    for w in v.iter() {
        println!("Weight: {}", w);
    }
    println!("--------------------------");
    println!("After sorting:");
    println!("--------------------------");
    v.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    for w in v.iter() {
        println!("Weight: {}", w);
    }
    petgraph_test();
}

fn petgraph_test() {
    use petgraph::algo::astar;
    use petgraph::Graph;
    type G = Graph<(f64, f64), f64, petgraph::Undirected>;

    let mut g = G::new_undirected();
    let a = g.add_node((0., 0.));
    let b = g.add_node((2., 0.));
    let c = g.add_node((1., 1.));
    let d = g.add_node((0., 2.));
    let e = g.add_node((3., 3.));
    let f = g.add_node((4., 2.));
    g.extend_with_edges(&[(a, d, 1.), (b, c, 1.), (b, f, 1.), (c, e, 1.), (d, e, 1.)]);

    // Graph represented with the weight of each edge
    // Edges with '*' are part of the optimal path.
    //
    //     2       1
    // a ----- b ----- c
    // | 4*    | 7     |
    // d       f       | 5
    // | 1*    | 1*    |
    // \------ e ------/

    let (length, path) = astar(&g, a, |finish| finish == f, |e| *e.weight(), |_| 0.).unwrap();
    let path_nodes: Vec<(f64, f64)> = path
        .iter()
        .map(|n| {
            if let Some((x, y)) = g.node_weight(*n) {
                (*x, *y)
            } else {
                (f64::INFINITY, f64::INFINITY)
            }
        })
        .collect();
    let path_edges: Vec<(Option<(f64, f64)>, Option<(f64, f64)>, Option<f64>)> = path
        .iter()
        .zip(path.iter().skip(1))
        .map(|(a, b)| {
            (
                g.node_weight(*a).cloned(),
                g.node_weight(*b).cloned(),
                if let Some(e) = g.find_edge(*a, *b) {
                    g.edge_weight(e).cloned()
                } else {
                    None
                },
            )
        })
        .collect();
    // assert_eq!(path, Some((6, vec![a, d, e, f])));
    println!("Path length: {}", length);
    for n in path.iter() {
        if let Some((x, y)) = g.node_weight(*n) {
            println!("Node: {}-{}", x, y);
        } else {
            println!("None");
        }
    }

    println!("Detailed:");
    for e in path_edges {
        let print_node = |n: Option<(f64, f64)>| {
            if let Some(n) = n {
                print!("{}-{}", n.0, n.1);
            } else {
                print!("{{unknown}}");
            }
        };
        print_node(e.0);
        if let Some(w) = e.2 {
            print!(" <-- {:^5.3} --> ", w);
        } else {
            print!(" <-----------> ");
        }
        print_node(e.1);
        println!();
    }
}
