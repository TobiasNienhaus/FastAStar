mod dijkstra;
mod graph_types;
use dijkstra::Graph as DGraph;
use graph_types::Node;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut graph = DGraph::new();
    let a = graph.add_vertex(Node::new(0., 0.));
    let b = graph.add_vertex(Node::new(1., 0.));
    let c = graph.add_vertex(Node::new(2., 0.));
    let d = graph.add_vertex(Node::new(0., 1.));
    let e = graph.add_vertex(Node::new(1., 1.));
    let f = graph.add_vertex(Node::new(2., 1.));
    let g = graph.add_vertex(Node::new(0., 2.));
    let h = graph.add_vertex(Node::new(1., 2.));
    let i = graph.add_vertex(Node::new(2., 2.));

    graph.add_edge(&a, &b).unwrap();
    graph.add_edge(&a, &d).unwrap();
    graph.add_edge(&b, &c).unwrap();
    graph.add_edge(&b, &f).unwrap();
    graph.add_edge(&c, &g).unwrap();
    graph.add_edge(&d, &e).unwrap();
    graph.add_edge(&d, &g).unwrap();
    graph.add_edge(&e, &f).unwrap();
    graph.add_edge(&e, &h).unwrap();
    graph.add_edge(&f, &i).unwrap();
    graph.add_edge(&g, &h).unwrap();
    graph.add_edge(&h, &i).unwrap();

    if let Some(p) = dijkstra::algo(&graph, &a, &i) {
        println!("Path:\n");
        for n in p {
            println!("Node: {:?}", graph.fetch(&n));
        }
    } else {
        println!("No Path!");
    }
    Ok(())
}
