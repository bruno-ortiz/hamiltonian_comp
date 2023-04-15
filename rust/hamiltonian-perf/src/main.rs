use std::time::Instant;

use bitset::BitSet;

use crate::graph::{Graph, GraphBuilder};

mod bitset;
mod graph;

pub fn find_hamiltonian(graph: &Graph, starting_node: &str) -> Option<Vec<String>> {
    let starting_node = graph.get_node_idx(starting_node);
    let mut path = Vec::with_capacity(graph.size());
    let mut bitset = BitSet::new();
    find_hamiltonian_r(graph, starting_node, &mut path, &mut bitset)
}

fn find_hamiltonian_r<'a>(
    graph: &'a Graph,
    current_node_idx: usize,
    path: &mut Vec<usize>,
    visited: &mut BitSet,
) -> Option<Vec<String>> {
    path.push(current_node_idx);
    visited.set(current_node_idx);

    if path.len() == graph.size() {
        return Some(graph.get_node_names(path));
    } else {
        let current_node = graph.get_node_by_index(current_node_idx);
        let mut to_visit: usize = current_node.0 & !visited.state;
        while to_visit != 0 {
            let next = to_visit.trailing_zeros() as usize;
            let r = find_hamiltonian_r(graph, next, path, visited);
            if r.is_some() {
                return r;
            }
            to_visit = to_visit & !(1 << next);
        }
    }

    visited.unset(current_node_idx);
    path.pop();
    None
}

fn main() {
    let mut graph_builder = GraphBuilder::default();
    graph_builder.link("wa", vec!["or", "id"]);
    graph_builder.link("or", vec!["wa", "id", "nv", "ca"]);
    graph_builder.link("ca", vec!["or", "nv", "az"]);
    graph_builder.link("id", vec!["wa", "or", "nv", "ut", "wy", "mt"]);
    graph_builder.link("nv", vec!["or", "ca", "az", "ut", "id"]);
    graph_builder.link("ut", vec!["id", "nv", "az", "co", "wy"]);
    graph_builder.link("az", vec!["ca", "nv", "ut", "nm"]);
    graph_builder.link("mt", vec!["id", "wy", "sd", "nd"]);
    graph_builder.link("wy", vec!["mt", "id", "ut", "co", "ne", "sd"]);
    graph_builder.link("co", vec!["wy", "ut", "nm", "ok", "ks", "ne"]);
    graph_builder.link("nm", vec!["co", "az", "tx", "ok"]);
    graph_builder.link("nd", vec!["mt", "sd", "mn"]);
    graph_builder.link("sd", vec!["nd", "mt", "wy", "ne", "ia", "mn"]);
    graph_builder.link("ne", vec!["sd", "wy", "co", "ks", "mo", "ia"]);
    graph_builder.link("ks", vec!["ne", "co", "ok", "mo"]);
    graph_builder.link("ok", vec!["ks", "co", "nm", "tx", "ar", "mo"]);
    graph_builder.link("tx", vec!["ok", "nm", "la", "ar"]);
    graph_builder.link("mn", vec!["nd", "sd", "ia", "wi"]);
    graph_builder.link("ia", vec!["mn", "sd", "ne", "mo", "il", "wi"]);
    graph_builder.link("mo", vec!["ia", "ne", "ks", "ok", "ar", "tn", "ky", "il"]);
    graph_builder.link("ar", vec!["mo", "ok", "tx", "la", "ms", "tn"]);
    graph_builder.link("la", vec!["ar", "tx", "ms"]);
    graph_builder.link("wi", vec!["mn", "ia", "il"]);
    graph_builder.link("il", vec!["wi", "ia", "mo", "ky", "in"]);
    graph_builder.link("tn", vec!["ky", "mo", "ar", "ms", "al", "ga", "nc", "va"]);
    graph_builder.link("ms", vec!["tn", "ar", "la", "al"]);
    graph_builder.link("mi", vec!["in", "oh"]);
    graph_builder.link("in", vec!["mi", "il", "ky", "oh"]);
    graph_builder.link("ky", vec!["oh", "in", "il", "mo", "tn", "va", "wv"]);
    graph_builder.link("al", vec!["tn", "ms", "fl", "ga"]);
    graph_builder.link("ga", vec!["nc", "tn", "al", "fl", "sc"]);
    graph_builder.link("oh", vec!["mi", "in", "ky", "wv", "pa"]);
    graph_builder.link("wv", vec!["pa", "oh", "ky", "va", "md"]);
    graph_builder.link("ny", vec!["pa", "nj", "ct", "ma", "vt"]);
    graph_builder.link("nj", vec!["ny", "pa", "de"]);
    graph_builder.link("pa", vec!["ny", "nj", "oh", "wv", "md", "de"]);
    graph_builder.link("va", vec!["md", "wv", "ky", "tn", "nc", "wdc"]);
    graph_builder.link("nc", vec!["va", "tn", "ga", "sc"]);
    graph_builder.link("sc", vec!["nc", "ga"]);
    graph_builder.link("fl", vec!["ga", "al"]);
    graph_builder.link("me", vec!["nh"]);
    graph_builder.link("nh", vec!["me", "vt", "ma"]);
    graph_builder.link("vt", vec!["nh", "ny", "ma"]);
    graph_builder.link("ma", vec!["nh", "vt", "ny", "ct", "ri"]);
    graph_builder.link("ct", vec!["ma", "ny", "ri"]);
    graph_builder.link("ri", vec!["ma", "ct"]);
    graph_builder.link("de", vec!["pa", "md", "nj"]);
    graph_builder.link("md", vec!["pa", "wv", "va", "de", "wdc"]);
    graph_builder.link("wdc", vec!["md", "va"]);

    let eastern = vec![
        "mn", "ia", "mo", "ar", "la", "wi", "il", "tn", "ms", "mi", "in", "ky", "al", "ga", "oh",
        "wv", "ny", "nj", "pa", "va", "nc", "sc", "fl", "me", "nh", "vt", "ma", "ct", "ri", "de",
        "md", "wdc",
    ];

    let graph = graph_builder.build(Some(eastern));
    let now = Instant::now();
    let path = find_hamiltonian(&graph, "ri");
    println!("found path: {:?}", path);
    println!("{}ms", now.elapsed().as_millis());
}
