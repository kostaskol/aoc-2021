use std::collections::HashMap;
use crate::utils::parsing::read_file;

type Graph = HashMap<String, Vec<String>>;

pub fn run(extra: bool, test: bool) -> String {
  let input = read_file("12", test);
  let graph = create_graph(input);

  format!("{}",
    match extra {
      false => p1::run(graph),
      true => p2::run(graph)
    }
  )
}

fn create_graph(input: Vec<String>) -> Graph {
  let mut graph: HashMap<String, Vec<String>> = HashMap::new();
  for line in input {
    let split_line = line.split('-').collect::<Vec<&str>>();

    let from = split_line[0].to_string();
    let to = split_line[1].to_string();

    graph.entry(from.clone()).or_insert_with(Vec::new).push(to.clone());
    if from != "start" {
      graph.entry(to).or_insert_with(Vec::new).push(from);
    }
  }

  graph
}

mod p1 {
  use std::collections::HashSet;
  use super::Graph;

  pub fn run(graph: Graph) -> i32 {
    // Only lowercase nodes go into the visited set
    let mut visited = HashSet::<String>::new();
    bfs(&graph, &mut visited, &mut vec![], "start".to_string())
  }

  fn bfs(
    graph: &Graph,
    visited: &mut HashSet<String>,
    mut path: &mut Vec<String>,
    node: String
  ) -> i32 {
    path.push(node.clone());

    if node.to_lowercase() == node {
      visited.insert(node.clone());
    }

    let mut sum = 0;
    if node == "end" {
      // Comment this in to also print the path followed
      // println!("{:?}", path);
      sum = 1;
    } else if let Some(next_set) = graph.get(&node) {
      for next in next_set {
        if !visited.contains(next) {
          sum += bfs(graph, visited, &mut path, next.to_string());
        }
      }
    }

    visited.remove(&node);
    sum
  }
}

mod p2 {
  use std::collections::HashSet;
  use super::Graph;

  pub fn run(graph: Graph) -> i32 {
    // Only lowercase nodes go into the visited set
    let mut visited = HashSet::<String>::new();
    let protected_nodes = lowercase_list(&graph);

    let mut all_paths: Vec<Vec<String>> = Vec::new();
    for protected_node in protected_nodes {
      bfs(&graph,
          &mut visited,
          &mut vec![],
          "start".to_string(),
          (protected_node, false),
          &mut all_paths);
    }

    all_paths.sort();
    all_paths.dedup();
    all_paths.len() as i32
  }

  fn bfs(
    graph: &Graph,
    visited: &mut HashSet<String>,
    mut path: &mut Vec<String>,
    node: String,
    mut protected: (String, bool),
    mut all_paths: &mut Vec<Vec<String>>
  ) {
    path.push(node.clone());

    if node == "end" {
      all_paths.push(path.clone());
    } else if let Some(next_set) = graph.get(&node) {
      if node.to_lowercase() == node {
        if node == protected.0 && !protected.1 {
          protected.1 = true;
        } else {
          visited.insert(node.clone());
        }
      }

      for next in next_set {
        if !visited.contains(next) {
          bfs(
            graph,
            visited,
            &mut path,
            next.to_string(),
            protected.clone(),
            &mut all_paths);
        }
      }
    }

    visited.remove(&node);
    path.pop();
  }

  fn lowercase_list(graph: &Graph) -> Vec<String> {
    let mut lowercase = vec![];
    for node in graph.keys() {
      if node.to_lowercase() == *node && !vec!["start", "end"].contains(&&node[..]) {
        lowercase.push(node.to_string());
      }
    }
    lowercase
  }
}

#[cfg(test)]
mod tests {
  use super::run;

  #[test]
  fn test_p1() {
    assert_eq!(run(false, true), "226");
  }

  #[test]
  fn test_p2() {
    assert_eq!(run(true, true), "3509");
  }
}