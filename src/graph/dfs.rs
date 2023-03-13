use std::collections::VecDeque;

struct Graph {
    n: usize,
    graph: Vec<Vec<usize>>,
}

impl Graph {
    fn new(size: usize) -> Self {
        Graph {
            n: size,
            graph: vec![Vec::new(); size],
        }
    }

    #[warn(dead_code)]
    fn add_graph(&mut self, x: usize, y: usize) {
        self.graph[x].push(y);
        self.graph[y].push(x);
    }

    #[warn(dead_code)]
    fn add_direct_graph(&mut self, x: usize, y: usize) {
        self.graph[x].push(y);
    }

    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut que: VecDeque<usize> = VecDeque::new();
        let mut dist: Vec<Option<usize>> = vec![None; self.n];
        dist[start] = Some(0);
        que.push_back(start);

        while let Some(v) = que.pop_front() {
            for e in self.graph[v].iter() {
                if dist[*e].is_some() {
                    continue;
                }

                dist[*e] = Some(dist[v].unwrap() + 1);
                que.push_back(*e);
            }
        }

        dist.into_iter()
            .map(|x| x.unwrap_or_default())
            .collect::<Vec<usize>>()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn is_graph() {
        let mut g = Graph::new(9);
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

        for (x, y) in a {
            g.add_graph(x, y);
        }

        assert_eq!(g.dfs(0), vec![0, 1, 1, 2, 1, 2, 3, 3, 2]);
    }
}
