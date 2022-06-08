use proconio::input;
use std::collections::VecDeque;

struct Node {
    dim: usize,
    edge_list: Vec<usize>,
}

impl Node {
    fn new() -> Node {
        Node {
            dim: 0,
            edge_list: Vec::new(),
        }
    }
    fn add_edge(&mut self, edge: usize) {
        self.edge_list.push(edge);
        self.dim += 1;
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edge: [(usize, usize); m],
        q: usize,
        query: [(usize, usize); q],
    }
    let mut graph = Vec::new();
    for _i in 0..n {
        graph.push(Node::new());
    }
    for (i, j) in edge {
        graph[i - 1].add_edge(j - 1);
        graph[j - 1].add_edge(i - 1);
    }
    for (i, j) in query {
        let mut look_list = VecDeque::new();
        look_list.push_back(i - 1);
        if j >= 1 {
            for k in &graph[i - 1].edge_list {
                if !look_list.contains(k) {
                    look_list.push_back(*k);
                }
            }
        }
        if j >= 2 {
            for k in &graph[i - 1].edge_list {
                if !look_list.contains(k) {
                    continue;
                }
                for l in &graph[*k].edge_list {
                    if !look_list.contains(l) {
                        look_list.push_back(*l);
                    }
                }
            }
        }
        if j == 3 {
            for k in &graph[i - 1].edge_list {
                if !look_list.contains(k) {
                    continue;
                }
                for l in &graph[*k].edge_list {
                    if !look_list.contains(l) {
                        continue;
                    }
                    for r in &graph[*l].edge_list {
                        if !look_list.contains(r) {
                            look_list.push_back(*r);
                        }
                    }
                }
            }
        }
        let mut ans = 0;
        for k in look_list {
            ans += k + 1;
        }
        println!("{}", ans);
    }
}
