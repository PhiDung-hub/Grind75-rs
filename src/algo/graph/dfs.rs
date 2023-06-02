use crate::structs::graph::*;

use std::collections::{HashSet, VecDeque};

pub fn depth_first_search(graph: &Graph, root: Node, target: Node) -> Option<Vec<u32>> {
    let mut visited: HashSet<Node> = HashSet::new();
    let mut history: Vec<u32> = Vec::new();
    let mut stack = VecDeque::new();

    visited.insert(root);
    stack.push_front(root);

    while let Some(currentnode) = stack.pop_front() {
        history.push(currentnode.value());

        // If we reach the goal, return our travel history.
        if currentnode == target {
            return Some(history);
        }

        // Check the neighboring nodes for any that we've not visited yet.
        for &neighbor in currentnode.neighbors(graph).iter().rev() {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                stack.push_front(neighbor);
            }
        }
    }

    // All nodes were visited, yet the target was not found.
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /* Example graph #1:
     *
     *            (1)   <--- Root
     *           /   \
     *         (2)   (3)
     *        / |     | \
     *     (4) (5)   (6) (7)
     *          |
     *         (8)
     */
    fn graph1() -> Graph {
        let nodes = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7), (5, 8)];

        Graph::new(nodes.into_iter().map(|v| v.into()).collect(), edges.into_iter().map(|e| e.into()).collect())
    }

    #[test]
    fn depth_first_search_graph1_when_node_not_found_returns_none() {
        let graph = graph1();
        let root = 1;
        let target = 10;

        assert_eq!(depth_first_search(&graph, root.into(), target.into()), None);
    }

    #[test]
    fn depth_first_search_graph1_when_target_8_should_evaluate_all_nodes_first() {
        let graph = graph1();
        let root = 1;
        let target = 8;

        let expected_path = vec![1, 2, 4, 5, 8];

        assert_eq!(depth_first_search(&graph, root.into(), target.into()), Some(expected_path));
    }

    /* Example graph #2:
     *
     *     (1) --- (2)     (3) --- (4)
     *            / |     /       /
     *          /   |   /       /
     *        /     | /       /
     *     (5)     (6) --- (7)     (8)
     */
    fn graph2() -> Graph {
        let nodes = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let undirected_edges = vec![
            (1, 2),
            (2, 1),
            (2, 5),
            (5, 2),
            (2, 6),
            (6, 2),
            (3, 4),
            (4, 3),
            (3, 6),
            (6, 3),
            (4, 7),
            (7, 4),
            (6, 7),
            (7, 6),
        ];

        Graph::new(nodes.into_iter().map(|v| v.into()).collect(), undirected_edges.into_iter().map(|e| e.into()).collect())
    }

    #[test]
    fn depth_first_search_graph2_when_no_path_to_node_returns_none() {
        let graph = graph2();
        let root = 8;
        let target = 4;

        assert_eq!(depth_first_search(&graph, root.into(), target.into()), None);
    }

    #[test]
    fn depth_first_search_graph2_should_find_path_from_4_to_1() {
        let graph = graph2();
        let root = 4;
        let target = 1;

        let expected_path = vec![4, 3, 6, 2, 1];

        assert_eq!(depth_first_search(&graph, root.into(), target.into()), Some(expected_path));
    }
}
