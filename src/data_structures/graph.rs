#[derive(Debug, PartialEq, Eq, Clone)]
struct Edge<T> {
    src_node: usize,
    dst_node: usize,
    weight: Option<T>,
}

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_graph() {
        let mut graph: Graph<i32> = Graph::new(9);

        let e = vec![
            (1, 2),
            (2, 3),
            (2, 7),
            (2, 8),
            (4, 5),
            (5, 6),
            (5, 7),
            (6, 7),
            (7, 8),
        ];
        for (a, b) in e.into_iter() {
            graph.add_edge(a, b, None);
            graph.add_edge(b, a, None);
        }
        println!("{:#?}", graph);
    }
}
