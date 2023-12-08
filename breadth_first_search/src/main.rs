use std::collections::HashSet;
use std::collections::VecDeque;

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_bfs() {
       let nodes = vec![Node(1), Node(2), Node(3), Node(4)];
       let edges = vec![Edge(1, 2), Edge(1, 3), Edge(2, 4)];
       let graph = Graph::new(nodes, edges);

       let result = graph.bfs(1);
       assert_eq!(result, vec![1, 2, 3, 4]);
   }

    #[test]
    fn test_single_node() {
    let nodes = vec![Node(1)];
    let edges = vec![];
    let graph = Graph::new(nodes, edges);

    let result = graph.bfs(1);
    assert_eq!(result, vec![1]);
    }

   #[test]
    fn test_disconnected_graph() {
    let nodes = vec![Node(1), Node(2), Node(3), Node(4)];
    let edges = vec![Edge(1, 2), Edge(3, 4)];
    let graph = Graph::new(nodes, edges);

    let result = graph.bfs(1);
    assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_tree_with_multiple_branches() {
    let nodes = vec![Node(1), Node(2), Node(3), Node(4), Node(5), Node(6)];
    let edges = vec![Edge(1, 2), Edge(1, 3), Edge(2, 4), Edge(2, 5), Edge(3, 6)];
    let graph = Graph::new(nodes, edges);

    let result = graph.bfs(1);
    assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

}

pub struct Node(u32);

pub struct Edge(u32, u32);

pub struct Graph {
  nodes: Vec<Node>,
  edges: Vec<Edge>,
}

impl Graph {
  pub fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
      Graph { nodes, edges }
  }

  pub fn bfs(&self, start: u32) -> Vec<u32> {
      let mut visited = vec![false; self.nodes.len()];
      let mut queue = VecDeque::new();
      let mut result = Vec::new();

      visited[(start - 1) as usize] = true;
      queue.push_back(start);

      while let Some(node) = queue.pop_front() {
          result.push(node);

          for edge in &self.edges {
              if edge.0 == node && !visited[(edge.1 - 1) as usize] {
                visited[(edge.1 - 1)as usize] = true;
                queue.push_back(edge.1);
              }
          }
      }

      result
  }
}

fn main() {
    let nodes = vec![Node(1), Node(2), Node(3), Node(4)];
    let edges = vec![Edge(1, 2), Edge(1, 3), Edge(2, 4)];
    let graph = Graph::new(nodes, edges);

    let result = graph.bfs(1);
    println!("{:?}", result)
}
