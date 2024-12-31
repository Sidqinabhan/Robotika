use std::collections::VecDeque;

fn find_path(grid: &Vec<Vec<i32>>, start: (usize, usize), end: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut visited = vec![vec![false; cols]; rows];
    let mut queue = VecDeque::new();
    let mut parent = vec![vec![None; cols]; rows];

    // Start BFS
    queue.push_back(start);
    visited[start.0][start.1] = true;

    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == end {
            // Reconstruct path
            let mut path = vec![];
            let mut current = Some((x, y));
            while let Some(pos) = current {
                path.push(pos);
                current = parent[pos.0][pos.1];
            }
            path.reverse();
            return Some(path);
        }

        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < rows as isize && ny >= 0 && ny < cols as isize {
                let (nx, ny) = (nx as usize, ny as usize);
                if !visited[nx][ny] && grid[nx][ny] == 0 {
                    visited[nx][ny] = true;
                    queue.push_back((nx, ny));
                    parent[nx][ny] = Some((x, y));
                }
            }
        }
    }

    None // No path found
}

fn main() {
    let grid = vec![
        vec![0, 0, 0, 1, 0],
        vec![1, 1, 0, 1, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0],
    ];

    let start = (0, 0);
    let end = (4, 4);

    match find_path(&grid, start, end) {
        Some(path) => {
            println!("Path found:");
            for (x, y) in path {
                println!("({}, {})", x, y);
            }
        }
        None => {
            println!("No path found!");
        }
    }
}
