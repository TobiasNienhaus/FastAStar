#![feature(map_first_last)]
mod astar;
mod dijkstra;
mod graph_types;
mod img;
mod pathfinding_result;
use graph_types::Graph;
use graph_types::Node;
use img::traits::*;
use pathfinding_result::PathfindingResult;

type R = Result<(), Box<dyn std::error::Error>>;

fn main() -> R {
    // dijkstra_test2()?;
    pf_test("test_images/graph2.png")?;
    pf_test("test_images/graph4.png")?;
    // img_test()?;
    Ok(())
}

fn pf_test(name: &str) -> R {
    let (ids, graph) = img::load_graph_from_img(name.into())?;
    let start_pos = (0usize, 0usize);
    let end_pos = (ids.width() - 1, ids.height() - 1);
    if let Some(start) = ids.get_2d(start_pos) {
        if let Some(end) = ids.get_2d(end_pos) {
            let results = vec![
                ("Dijkstra", dijkstra::solve(&graph, start, end)),
                ("A*", astar::solve(&graph, start, end)),
            ];
            for (name, res) in results.iter() {
                println!(
                    "{} has {}solved the problem, visiting {} of {} nodes",
                    name,
                    if res.is_solved() { "" } else { "not" },
                    res.visited(),
                    graph.vertex_count()
                );
                if res.is_solved() {
                    let p: Vec<(usize, usize)> = res
                        .path()
                        .unwrap()
                        .iter()
                        .map(|id| {
                            let pos = graph.fetch(id).unwrap();
                            (pos.x() as usize, pos.y() as usize)
                        })
                        .collect();
                    print_grid(start_pos, end_pos, &ids, &p);
                }
            }
            assert!(
                results.iter().all(|r| r.1.is_solved()) || results.iter().all(|r| !r.1.is_solved())
            )
        }
    }
    Ok(())
}

fn print_grid(
    start: (usize, usize),
    end: (usize, usize),
    grid: &img::IdGrid,
    path: &Vec<(usize, usize)>,
) {
    const GRIDTILE: &str = "░░";
    const PATHTILE: &str = "██";
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            if let Some(_) = grid.get_2d((x, y)) {
                if x == start.0 && y == start.1 {
                    print!("S ");
                } else if x == end.0 && y == end.1 {
                    print!("E ");
                } else if path.contains(&(x, y)) {
                    print!("{}", PATHTILE);
                } else {
                    print!("{}", GRIDTILE);
                }
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

// fn img_test() -> R {
//     println!("Grayscale:");
//     img::test_img("test_images/image_test.png".into())?;
//     println!("RGB:");
//     img::test_img("test_images/image_test2.png".into())?;
//     println!("Grayscale Grid:");
//     img::test_grid("test_images/image_test.png".into(), 128)?;
//     println!("RGB Grid:");
//     img::test_grid("test_images/image_test2.png".into(), 70)?;
//     println!("Bigger:");
//     img::test_grid("test_images/graph2.png".into(), 128)?;
//     println!("Biggest:");
//     img::test_grid("test_images/graph3.png".into(), 128)?;
//     println!("Rect:");
//     img::test_grid("test_images/graph4.png".into(), 128)?;
//     Ok(())
// }

// fn dijkstra_test2(name: &str) -> R {
//     let (ids, graph) = img::load_graph_from_img(name.into())?;
//     if let Some(start) = ids.get_2d((0, 0)) {
//         if let Some(end) = ids.get_2d((ids.width() - 1, ids.height() - 1)) {
//             println!("Start: {:?}", graph.fetch(start));
//             println!("End: {:?}", graph.fetch(end));
//             if let Some(p) = dijkstra::solve(&graph, start, end) {
//                 println!("Path:");
//                 for n in p.iter() {
//                     println!("Node: {:?}", graph.fetch(&n));
//                 }
//                 println!("--> Length: {}", p.len());
//             } else {
//                 println!("No Path!");
//             }
//         }
//     }
//     Ok(())
// }

// fn _dijkstra_test() {
//     let mut graph = Graph::new();
//     let a = graph.add_vertex(Node::new(0., 0.));
//     let b = graph.add_vertex(Node::new(1., 0.));
//     let c = graph.add_vertex(Node::new(2., 0.));
//     let d = graph.add_vertex(Node::new(0., 1.));
//     let e = graph.add_vertex(Node::new(1., 1.));
//     let f = graph.add_vertex(Node::new(2., 1.));
//     let g = graph.add_vertex(Node::new(0., 2.));
//     let h = graph.add_vertex(Node::new(1., 2.));
//     let i = graph.add_vertex(Node::new(2., 2.));

//     graph.add_edge(&a, &b).unwrap();
//     graph.add_edge(&a, &d).unwrap();
//     graph.add_edge(&b, &c).unwrap();
//     graph.add_edge(&b, &e).unwrap();
//     graph.add_edge(&c, &f).unwrap();
//     graph.add_edge(&d, &e).unwrap();
//     graph.add_edge(&d, &g).unwrap();
//     graph.add_edge(&e, &f).unwrap();
//     graph.add_edge(&e, &h).unwrap();
//     graph.add_edge(&f, &i).unwrap();
//     graph.add_edge(&g, &h).unwrap();
//     graph.add_edge(&h, &i).unwrap();

//     if let Some(p) = dijkstra::solve(&graph, &a, &i) {
//         println!("Path:");
//         for n in p.iter() {
//             println!("Node: {:?}", graph.fetch(&n));
//         }
//         println!("--> Length: {}", p.len());
//     } else {
//         println!("No Path!");
//     }
// }
