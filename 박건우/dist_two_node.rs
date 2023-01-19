use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let (n, m) = {
        let mut v: Vec<u32> = Vec::new();
        for i in input().split_whitespace() {
            v.push(i.parse().unwrap());
        }
        (v[0], v[1])
    };

    let mut graph: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    for _ in 0..n - 1 {
        let (start, next, dist) = {
            let mut v: Vec<u32> = Vec::new();
            for i in input().split_whitespace() {
                v.push(i.parse().unwrap());
            }
            (v[0], v[1], v[2])
        };
        graph.entry(start).or_insert(Vec::new()).push((next, dist));
        graph.entry(next).or_insert(Vec::new()).push((start, dist));
    }

    let mut wtk: Vec<(u32, u32)> = Vec::new();
    for _ in 0..m {
        let (start, dist) = {
            let mut v: Vec<u32> = Vec::new();
            for i in input().split_whitespace() {
                v.push(i.parse().unwrap());
            }
            (v[0], v[1])
        };
        wtk.push((start, dist));
    }
    println!("Graph: {:?}", graph);
    println!("WTK: {:?}", wtk);

    for (start, dest) in wtk.into_iter() {
        let s = std::time::Instant::now();
        println!("DFS: {}", depth_first_search(&graph, start, dest));
        println!("Time: {:?}", s.elapsed());
        let s = std::time::Instant::now();
        println!("BFS: {}", breadth_first_search(&graph, start, dest));
        println!("Time: {:?}", s.elapsed());
    }
}

fn depth_first_search(graph: &HashMap<u32, Vec<(u32, u32)>>, start: u32, dest: u32) -> u32 {
    let mut visited: HashSet<u32> = HashSet::new();
    let mut stack: VecDeque<(u32, u32)> = VecDeque::new();
    stack.push_back((start, 0));

    while !stack.is_empty() {
        let (node, dist) = stack.pop_back().unwrap();

        if node == dest {
            return dist;
        }

        visited.insert(node);

        for (next, next_dist) in graph[&node].iter() {
            if !visited.contains(next) {
                stack.push_back((*next, dist + next_dist));
            }
        }
    }
    0
}

fn breadth_first_search(graph: &HashMap<u32, Vec<(u32, u32)>>, start: u32, dest: u32) -> u32 {
    let mut visited: HashSet<u32> = HashSet::new();
    let mut queue: VecDeque<(u32, u32)> = VecDeque::new();
    queue.push_back((start, 0));

    while !queue.is_empty() {
        let (node, dist) = queue.pop_front().unwrap();

        if node == dest {
            return dist;
        }

        visited.insert(node);

        for (next, next_dist) in graph[&node].iter() {
            if !visited.contains(next) {
                queue.push_back((*next, dist + next_dist));
            }
        }
    }
    0
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
