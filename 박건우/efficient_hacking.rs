use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    let (_, m) = {
        let mut v: Vec<u32> = Vec::new();
        for i in input().split_whitespace() {
            v.push(i.parse().unwrap());
        }
        (v[0], v[1])
    };

    for _ in 0..m {
        let (node1, node2) = {
            let mut v: Vec<u32> = Vec::new();
            for i in input().split_whitespace() {
                v.push(i.parse().unwrap());
            }
            (v[0], v[1])
        };
        graph.entry(node2).or_insert(Vec::new()).push(node1);
    }

    let mut computers: Vec<u32> = Vec::new();
    let mut max: u32 = 0;
    for &node in graph.keys() {
        let count = depth_first_search(&graph, node);
        match max.cmp(&count) {
            std::cmp::Ordering::Less => {
                max = count;
                computers.clear();
                computers.push(node);
            }
            std::cmp::Ordering::Equal => {
                computers.push(node);
            }
            _ => (),
        }
    }
    computers.sort();

    // println!("Graph: {:?}", graph);
    // println!("computers: {:?}", computers);
    for computer in computers {
        print!("{} ", computer);
    }
}

fn depth_first_search(graph: &HashMap<u32, Vec<u32>>, start: u32) -> u32 {
    let mut visited: HashSet<u32> = HashSet::new();
    let mut queue: VecDeque<u32> = VecDeque::new();
    let mut count: u32 = 0;
    queue.push_back(start);

    while !queue.is_empty() {
        let node = queue.pop_back().unwrap();

        visited.insert(node);
        count += 1;

        if graph.contains_key(&node) {
            for &next in graph[&node].iter() {
                if !visited.contains(&next) {
                    queue.push_back((next));
                }
            }
        }
    }
    return count;
}

fn input() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}
