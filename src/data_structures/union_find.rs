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

    fn union(&mut self, x: usize, y: usize) {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_union_find() {
        todo!();
    }
}
