use std::collections::{HashMap, HashSet};

use aoc_2024::read_input_v1;
use itertools::Itertools;

#[derive(Debug)]
struct Graph {
    am: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn from_input(input: &str) -> Self {
        let edges = input
            .lines()
            .flat_map(|l| {
                let (v1, v2) = l.split_once("-").unwrap();
                let id1 = v1.chars().fold(0, |acc, cur| acc * 256 + cur as usize);
                let id2 = v2.chars().fold(0, |acc, cur| acc * 256 + cur as usize);

                [(id1, id2), (id2, id1)]
            })
            .collect::<HashSet<_>>();

        let mut am = HashMap::new();

        for &edge in edges.iter().sorted() {
            am.entry(edge.0).or_insert_with(HashSet::new).insert(edge.1);
            am.entry(edge.1).or_insert_with(HashSet::new).insert(edge.0);
        }

        let am = am
            .iter()
            .map(|(k, v)| {
                let mut v = Vec::from_iter(v.iter().copied());
                v.sort();
                (*k, v)
            })
            .collect();

        Self { am }
    }

    fn max_clique(&self) -> String {
        let keys = self.am.keys().sorted().copied().collect::<Vec<_>>();

        let best = self.max_clique_with(&[], &keys);

        best.into_iter().map(to_str).join(",")
    }

    fn max_clique_with(&self, with: &[usize], rest: &[usize]) -> Vec<usize> {
        let mut best = with.to_vec();

        for &v in rest {
            let mut with = with.to_vec();
            with.push(v);

            let rest = rest
                .iter()
                .filter(|&x| *x > v && self.am[&v].contains(x))
                .copied()
                .collect::<Vec<_>>();

            let clique = self.max_clique_with(&with, &rest);

            if clique.len() > best.len() {
                best = clique;
            }
        }

        best
    }
}

fn to_str(w: usize) -> String {
    [((w >> 8) & 0xff) as u8 as char, (w & 0xff) as u8 as char]
        .into_iter()
        .collect()
}

fn main() {
    let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
    let input = &read_input_v1(23);

    let graph = Graph::from_input(input);

    println!("{:?}", graph.max_clique());
}
