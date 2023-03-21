pub struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self { parent: (0..n).collect() }
    }
    pub fn union(&mut self, from: usize, to: usize) {
        let from = self.find(from);
        let to = self.find(to);
        self.parent[to] = from;
    }
    pub fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
}
