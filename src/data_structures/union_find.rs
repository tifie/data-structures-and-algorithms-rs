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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_union_find() {
        let n = 8;
        let _q = 9;
        let query = vec![
            (0, 1, 2),
            (0, 3, 2),
            (1, 1, 3),
            (1, 1, 4),
            (0, 2, 4),
            (1, 4, 1),
            (0, 4, 2),
            (0, 0, 0),
            (1, 0, 0),
        ];

        let mut uf = UnionFind::new(n);
        let mut ans = Vec::new();

        for (q, a, b) in query {
            if q != 1 {
                uf.unite(a, b);
            } else {
                ans.push(if uf.is_same(a, b) { "Yes" } else { "No" });
            }
        }
        assert_eq!(ans, vec!["Yes", "No", "Yes", "Yes"]);
    }
}
