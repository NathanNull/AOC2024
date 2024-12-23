use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::helpers::first_n;

pub fn main(input: String, pt1: bool) {
    let connections_vec = input
        .lines()
        .map(|l| first_n::<2, _>(&mut l.split("-")))
        .collect::<Vec<_>>();
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
    for [first, second] in connections_vec {
        for [c1, c2] in [[first, second], [second, first]] {
            if !connections.contains_key(c1) {
                connections.insert(c1, HashSet::from_iter([c2]));
            } else {
                connections.get_mut(c1).unwrap().insert(c2);
            }
        }
    }
    if pt1 {
        let mut tris = 0;
        for (first, conns) in &connections {
            if first.starts_with('t') {
                for [c1, c2] in conns
                    .iter()
                    .combinations(2)
                    .map(|p| first_n(&mut p.iter().map(|s| **s)))
                {
                    if ![c1, c2].iter().any(|c| c.starts_with('t') && c > first)
                        && connections
                            .get(c1)
                            .is_some_and(|interlink| interlink.contains(c2))
                    {
                        //println!("Tri with {first}, {c1}, {c2}");
                        tris += 1;
                    }
                }
            }
        }
        println!("Triangles containing ts are {tris}");
    } else {
        let mut rets = vec![];
        bron_kerbosch(
            &connections,
            HashSet::new(),
            connections.iter().map(|(k, _)| *k).collect(),
            HashSet::new(),
            &mut rets,
        );
        //println!("Rets are {rets:?}");
        let mut max_len = rets
            .iter()
            .max_by_key(|n| n.len())
            .unwrap()
            .iter()
            .collect::<Vec<_>>();
        max_len.sort();
        println!("Max is {:?}", max_len.into_iter().join(","));
    }
}

fn bron_kerbosch<'a>(
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    r: HashSet<&'a str>,
    mut p: HashSet<&'a str>,
    mut x: HashSet<&'a str>,
    rets: &mut Vec<HashSet<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        rets.push(r);
    } else {
        let pivot = p.union(&x).max_by_key(|s| graph.get(*s).unwrap().len()).unwrap();
        for v in p.clone().difference(graph.get(pivot).unwrap()) {
            let n = graph.get(v).unwrap();
            bron_kerbosch(
                graph,
                r.union(&HashSet::from_iter([*v])).map(|n| *n).collect(),
                p.intersection(n).map(|n| *n).collect(),
                x.intersection(n).map(|n| *n).collect(),
                rets,
            );
            p.remove(v);
            x.insert(v);
        }
    }
}
