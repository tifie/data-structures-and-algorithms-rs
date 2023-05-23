#[derive(Debug, PartialEq, Eq, Clone)]
struct Edge<T> {
    src_node: usize,
    dst_node: usize,
    weight: Option<T>,
}

const INF: usize = 100000;

#[derive(Debug)]
struct Graph<T> {
    edges: Vec<Vec<Edge<T>>>,
}

impl<T: Clone> Graph<T> {
    fn new(n: usize) -> Self {
        Graph {
            edges: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src_node: usize, dst_node: usize, weight: Option<T>) {
        self.edges[src_node].push(Edge {
            src_node,
            dst_node,
            weight,
        });
    }

    fn dfs(&mut self, start: usize) -> Vec<usize> {
        let mut dist = vec![INF; self.edges.len()];
        let mut stack = Vec::<usize>::new();
        dist[start] = 0;
        stack.push(start);
        while let Some(node) = stack.pop() {
            for e in &self.edges[node] {
                if dist[e.dst_node] != INF {
                    continue;
                }
                stack.push(e.dst_node);
                dist[e.dst_node] = dist[e.src_node] + 1;
            }
        }

        dist
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn is_dfs() {
        let mut graph: Graph<i32> = Graph::new(9);
        let a: Vec<(usize, usize)> = vec![
            (0, 1),
            (0, 4),
            (0, 2),
            (1, 4),
            (1, 3),
            (1, 8),
            (2, 5),
            (3, 8),
            (4, 8),
            (5, 8),
            (5, 8),
            (5, 6),
            (3, 7),
            (6, 7),
        ];

        for (a, b) in a {
            graph.add_edge(a, b, None);
            graph.add_edge(b, a, None);
        }
        println!("{:?}", graph.dfs(0));
        assert_eq!(graph.dfs(0), vec![0, 1, 1, 2, 1, 2, 3, 3, 2]);
    }
}
