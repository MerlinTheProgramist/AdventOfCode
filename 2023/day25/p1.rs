use petgraph::algo::{astar, connected_components};
// use petgraph::dot::{Config, Dot};
use petgraph::graph::UnGraph;
// use std::collections::hash_map::Entry;
use itertools::Itertools;
use petgraph::visit::Dfs;
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let mut graph = UnGraph::new_undirected();
    // Contsturct the graph from input file
    {
        let mut nodes = HashMap::new();

        for line in read_to_string("p.in").unwrap().lines() {
            let (source, targets) = line.split_once(": ").unwrap();
            let source = source.to_owned();
            let targets = targets.split_whitespace().map(|s| s.to_owned());

            nodes
                .entry(source.clone())
                .or_insert_with(|| graph.add_node(source.clone()));

            for target in targets {
                nodes
                    .entry(target.clone())
                    .or_insert_with(|| graph.add_node(target.clone()));
                graph.add_edge(nodes[&source], nodes[&target], ());
            }
        }
    }

    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    // count how many times we crossed the edge
    let mut counter = HashMap::<_, usize>::new();

    // for each pair of nodes find the shortest between them
    graph
        .node_indices()
        .take(10)
        .cartesian_product(graph.node_indices())
        .for_each(|(a, b)| {
            astar(&graph, a, |node| node == b, |_| 1, |_| 0)
                .unwrap()
                .1
                .iter()
                .tuple_windows::<(_, _)>()
                .map(|(&a, &b)| graph.find_edge(a, b).unwrap())
                .for_each(|edge| *counter.entry(edge).or_default() += 1)
        });

    // sort edges from the heaviest
    let heavy_edges = counter
        .iter()
        .sorted_by(|(_, a), (_, b)| b.cmp(a))
        .map(|(edge, _)| *edge)
        .collect_vec();

    println!("RESULTING!");
    // try each edge triplet, to find which splits the graph, starting from the heaviest
    let ans = (0..3)
        .map(|_| heavy_edges.iter())
        .multi_cartesian_product()
        .filter(|edges| edges.iter().unique().count() == 3)
        .find_map(|edges| {
            let mut graph2 = graph.clone();
            for edge in edges {
                graph2.remove_edge(*edge);
            }

            if connected_components(&graph2) != 2 {
                println!("not 2");
                return None;
            }

            let mut dfs = Dfs::new(&graph2, graph.node_indices().next().unwrap());
            let mut count = 0;
            while dfs.next(&graph2).is_some() {
                count += 1;
                println!("a");
            }
            Some(count * (graph.node_count() - count))
        })
        .unwrap();

    println!("{}", ans);
}
