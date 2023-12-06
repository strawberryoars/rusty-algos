use std::collections::HashSet;
use std::collections::VecDeque;

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_bfs() {
    let graph = vec![
        vec![1, 2],
        vec![0, 3, 4],
        vec![0, 3, 4],
        vec![1, 2],
        vec![1, 2],
    ];

    let result = bfs(&graph, 0);
    assert_eq!(result, vec![0, 1, 2, 3, 4]);
   }
}

fn bfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::new();
    let mut result = Vec::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        result.push(node);

        for &neighbor in &graph[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }

    result
}

fn main() {
    let graph = vec![
        vec![1, 2],
        vec![0, 3, 4],
        vec![0, 3, 4],
        vec![1, 2],
        vec![1, 2],
    ];

    let result = bfs(&graph, 1);
    println!("{:?}", result);
}
