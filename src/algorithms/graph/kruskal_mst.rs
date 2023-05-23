#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect::<Vec<_>>(),
            rank: vec![0; n],
        }
    }

    fn is_root(&self, x: usize) -> bool {
        self.parent[x] == x
    }

    fn find(&mut self, x: usize) -> usize {
        if self.is_root(x) {
            x
        } else {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
            root
        }
    }

    fn unite(&mut self, x: usize, y: usize) {
        let (root_x, root_y) = (self.find(x), self.find(y));

        if root_x != root_y {
            match self.rank[root_x].cmp(&self.rank[root_y]) {
                std::cmp::Ordering::Less => self.parent[root_x] = root_y,
                std::cmp::Ordering::Equal => {
                    self.parent[root_x] = root_y;
                    self.rank[root_y] += 1
                }
                std::cmp::Ordering::Greater => self.parent[root_y] = root_x,
            }
        }
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Edge<T> {
    src_node: usize,
    dst_node: usize,
    weight: Option<T>,
}

#[derive(Debug, Clone)]
struct Graph<T> {
    edges: Vec<Vec<Edge<T>>>,
}

impl<T> Graph<T>
where
    T: Clone + Ord + Copy,
{
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

    fn kruskal(&self) -> Graph<T> {
        let mut uf = UnionFind::new(self.edges.len());

        let mut all_edges = self
            .edges
            .clone()
            .into_iter()
            .flatten()
            .collect::<Vec<Edge<T>>>();

        all_edges.sort_by_key(|e| e.weight);

        let mut graph_mst = Graph::new(self.edges.len());

        for e in all_edges {
            if !uf.is_same(e.src_node, e.dst_node) {
                uf.unite(e.src_node, e.dst_node);
                graph_mst.add_edge(e.src_node, e.dst_node, e.weight);
            }
        }

        graph_mst
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn is_minimum_spanning_tree() {
        let mut graph = Graph::<usize>::new(7);
        let a = vec![
            (0, 1, 2),
            (0, 3, 5),
            (0, 2, 3),
            (1, 3, 7),
            (2, 4, 2),
            (3, 4, 15),
            (3, 5, 1),
            (4, 6, 11),
            (5, 6, 8),
        ];

        for (u, v, w) in a {
            graph.add_edge(u, v, Some(w));
            graph.add_edge(v, u, Some(w));
        }
        let mst = graph.kruskal();
        assert_eq!(
            mst.edges
                .iter()
                .flatten()
                .map(|e| e.weight.unwrap())
                .sum::<usize>(),
            21
        );
    }
}
