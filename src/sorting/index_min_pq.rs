#[derive(Default)]
pub struct IndexMinPQ<E> {
    pq: Vec<usize>,   // postion to elements
    pos: Vec<usize>,  // index to position in PQ
    elements: Vec<E>, // index to element
}

impl<E: PartialOrd + Clone + Default> IndexMinPQ<E> {
    pub fn new(size: usize) -> Self {
        IndexMinPQ {
            pq: vec![0; 1], // first slot not used
            pos: vec![0; size],
            elements: vec![E::default(); size],
        }
    }

    pub fn len(&self) -> usize {
        self.pq.len() - 1
    }
    pub fn is_empty(&self) -> bool {
        self.pq.len() <= 1
    }
    pub fn get(&self, i: usize) -> Option<&E> {
        if self.pos[i] != 0 {
            Some(&self.elements[i])
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<(usize, E)> {
        if !self.is_empty() {
            let i = self.pq.swap_remove(1);
            let v = self.elements[i].clone();
            self.pos[i] = 0;
            self.sink(1);
            Some((i, v))
        } else {
            None
        }
    }

    pub fn upsert(&mut self, i: usize, e: E) {
        if self.pos[i] != 0 {
            let old = self.elements[i].clone();
            self.elements[i] = e.clone();
            if e < old {
                self.swim(self.pos[i]);
            } else if old < e {
                self.sink(self.pos[i]);
            }
        } else {
            self.pos[i] = self.pq.len();
            self.elements[i] = e;
            self.pq.push(i);
            self.swim(self.len() - 1);
        }
    }

    fn sink(&mut self, mut i: usize) {
        while 2 * i <= self.len() {
            let mut j = 2 * i;
            if j < self.len() && self.less(j + 1, j) {
                j += 1;
            }
            // caution: !less() means >= not >
            if !self.less(j, i) {
                break;
            }

            self.swap(i, j);
            i = j;
        }
    }
    fn swim(&mut self, mut i: usize) {
        while i > 1 && self.less(i, i / 2) {
            self.swap(i, i / 2);
            i /= 2;
        }
    }

    fn less(&self, i: usize, j: usize) -> bool {
        self.elements[self.pq[i]] < self.elements[self.pq[j]]
    }
    fn swap(&mut self, i: usize, j: usize) {
        self.pq.swap(i, j);
        self.pos[self.pq[i]] = i;
        self.pos[self.pq[j]] = j;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut pq = IndexMinPQ::<usize>::new(10);
        assert_eq!(0, pq.len());
        assert_eq!(None, pq.pop());
    }

    #[test]
    fn one() {
        let mut pq = IndexMinPQ::<f64>::new(10);
        pq.upsert(1, 0.9);
        assert_eq!(1, pq.len());
        assert_eq!(Some((1, 0.9)), pq.pop());
        assert_eq!(None, pq.pop());
    }

    #[test]
    fn one_string() {
        let mut pq = IndexMinPQ::<&str>::new(10);
        pq.upsert(2, "abc");
        assert_eq!(1, pq.len());
        assert_eq!(Some((2, "abc")), pq.pop());
        assert_eq!(None, pq.pop());
    }

    #[test]
    fn several_elements() {
        let mut pq = IndexMinPQ::<isize>::new(10);
        pq.upsert(1, 9);
        pq.upsert(3, 2);
        pq.upsert(2, 3);
        assert_eq!(3, pq.len());
        assert_eq!(Some((3, 2)), pq.pop());
        assert_eq!(Some((2, 3)), pq.pop());
        assert_eq!(Some((1, 9)), pq.pop());
        assert_eq!(None, pq.pop());
    }

    #[test]
    fn push_and_pop() {
        let mut pq = IndexMinPQ::<isize>::new(10);
        pq.upsert(1, 2);
        pq.upsert(2, -1);
        pq.upsert(3, 4);
        assert_eq!(Some((2, -1)), pq.pop());
        pq.upsert(5, 1);
        pq.upsert(6, 3);

        assert_eq!(4, pq.len());
        assert_eq!(Some((5, 1)), pq.pop());
        assert_eq!(3, pq.len());
        assert_eq!(Some((1, 2)), pq.pop());
        assert_eq!(2, pq.len());
        assert_eq!(Some((6, 3)), pq.pop());
        assert_eq!(1, pq.len());
        assert_eq!(Some((3, 4)), pq.pop());
        assert_eq!(0, pq.len());
        assert_eq!(None, pq.pop());
        assert_eq!(0, pq.len());
    }

    #[test]
    fn with_identical_keys() {
        let mut pq = IndexMinPQ::<isize>::new(10);
        pq.upsert(1, 2);
        pq.upsert(2, 2);
        pq.upsert(4, 4);
        pq.upsert(3, 4);
        pq.upsert(0, 3);

        assert_eq!(5, pq.len());
        assert_eq!(Some((1, 2)), pq.pop());
        assert_eq!(Some((2, 2)), pq.pop());
        assert_eq!(3, pq.len());
        assert_eq!(Some((0, 3)), pq.pop());
        assert_eq!(Some((4, 4)), pq.pop());
        assert_eq!(Some((3, 4)), pq.pop());
        assert_eq!(0, pq.len());
        assert_eq!(None, pq.pop());
        assert_eq!(0, pq.len());
    }

    #[test]
    fn update() {
        let mut pq = IndexMinPQ::<isize>::new(10);
        pq.upsert(1, 2);
        pq.upsert(2, 3);
        pq.upsert(4, 4);
        pq.upsert(3, 5);

        pq.upsert(2, 1);
        pq.upsert(4, 9);
        assert_eq!(4, pq.len());
        assert_eq!(Some((2, 1)), pq.pop());
        assert_eq!(3, pq.len());
        assert_eq!(Some((1, 2)), pq.pop());
        assert_eq!(2, pq.len());
        assert_eq!(Some((3, 5)), pq.pop());
        assert_eq!(1, pq.len());
        assert_eq!(Some((4, 9)), pq.pop());
        assert_eq!(0, pq.len());
        assert_eq!(None, pq.pop());
        assert_eq!(0, pq.len());
    }
}
