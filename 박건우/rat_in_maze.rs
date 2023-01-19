fn main() {
    let (n, m) = {
        let mut v: Vec<u32> = Vec::new();
        for i in input().split_whitespace() {
            v.push(i.parse().unwrap());
        }
        (v[0], v[1])
    };

    let mut maze: Vec<Vec<i32>> = Vec::new();

    for _ in 0..n {
        for _ in 0..m {
            maze.push({
                let mut v: Vec<i32> = Vec::new();
                for val in input().split_terminator("").skip(1) {
                    v.push(val.parse().unwrap());
                }
                v
            });
        }
    }
    for i in 0..n {
        for j in 0..m {
            print!("{} ", maze[i][j]);
        }
        println!("");
    }

    let mut path: Vec<(usize, usize)> = vec![(0,0)];
    let mut visited: Vec<Vec<bool>> = vec![vec![false; m as usize]; n as usize];
    let mut res: Vec<Vec<(usize, usize)>> = Vec::new();
    rat_in_maze(&mut maze, &mut path, &mut visited, &mut res);
    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            print!("{} ", maze[i][j]);
        }
        println!();
    }
    println!("{:?}", res.len());
}

fn rat_in_maze(
    maze: &mut Vec<Vec<i32>>,
    path: &mut Vec<(usize, usize)>,
    visited: &mut Vec<Vec<bool>>,
    res: &mut Vec<Vec<(usize, usize)>>,
) {
    let (x, y) = path[path.len() - 1];

    if x == maze.len() - 1 && y == maze[0].len() - 1 {
        res.push(path.clone());
        return;
    }

    //left
    if x > 0 && maze[x - 1][y] == 1 && !visited[x - 1][y] {
        path.push((x - 1, y));
        visited[x - 1][y] = true;
        rat_in_maze(maze, path, visited, res);
        path.pop();
        visited[x - 1][y] = false;
    }
    //right
    if x < maze.len() - 1 && maze[x + 1][y] == 1 && !visited[x + 1][y] {
        path.push((x + 1, y));
        visited[x + 1][y] = true;
        rat_in_maze(maze, path, visited, res);
        path.pop();
        visited[x + 1][y] = false;
    }
    //down
    if y > 0 && maze[x][y - 1] == 1 && !visited[x][y - 1] {
        path.push((x, y - 1));
        visited[x][y - 1] = true;
        rat_in_maze(maze, path, visited, res);
        path.pop();
        visited[x][y - 1] = false;
    }
    //up
    if y < maze[0].len() - 1 && maze[x][y + 1] == 1 && !visited[x][y + 1] {
        path.push((x, y + 1));
        visited[x][y + 1] = true;
        rat_in_maze(maze, path, visited, res);
        path.pop();
        visited[x][y + 1] = false;
    }
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
