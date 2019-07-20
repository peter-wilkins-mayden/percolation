use std::vec::Vec;

#[derive(Debug)]
pub struct QuickFind {
    id: Vec<usize>,
    size: Vec<usize>,
}

impl QuickFind {
    pub fn new(n: usize) -> QuickFind {
        QuickFind {
            id: (0..n).collect(),
            size: vec![1; n],
        }
    }
    fn root(&mut self, i: usize) -> usize {
        let mut i = i;
        while i != self.id[i] {
            self.id[i] = self.id[self.id[i]]; // compress paths
            i = self.id[i]
        };
        i
    }
    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }
    pub fn union(&mut self, p: usize, q: usize) {
        let root_p = self.root(p);
        let root_q = self.root(q);

        if root_p == root_q { return; }

        if self.size[root_p] < self.size[root_q] {
            self.id[root_p] = root_q;
            self.size[root_q] += self.size[root_p];
        } else {
            self.id[root_q] = root_p;
            self.size[root_p] += self.size[root_q];
        }
    }
}

#[cfg(test)]
mod test {
    use super::QuickFind;

    #[test]
    fn basics() {
        let mut qf = QuickFind::new(5);
        qf.union(1 ,4);
        qf.union(0 ,1);
        qf.union(4 ,3);
        assert!(qf.connected(1, 4));
        assert_eq!(qf.connected(2,3), false);
        qf.union(2 ,0);
        assert!(qf.connected(2,3));
    }
}