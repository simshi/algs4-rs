#[derive(Default)]
pub struct MaxPQ<T> {
    pq: Vec<T>,
}

impl<T: PartialOrd> MaxPQ<T> {
    pub fn new() -> Self {
        MaxPQ { pq: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.pq.len()
    }
    pub fn is_empty(&self) -> bool {
        self.pq.len() == 0
    }

    pub fn pop(&mut self) -> Option<T> {
        if !self.is_empty() {
            (Some(self.pq.swap_remove(0)), self.sink(0)).0
        } else {
            None
        }
    }

    pub fn push(&mut self, e: T) {
        self.pq.push(e);
        self.swim(self.len() - 1);
    }

    fn sink(&mut self, mut i: usize) {
        while 2 * i + 1 < self.len() {
            let mut j = 2 * i + 1;
            if j + 1 < self.len() && self.less(j, j + 1) {
                j += 1;
            }
            // caution: !less() means >= not >
            if !self.less(i, j) {
                break;
            }

            self.pq.swap(i, j);
            i = j;
        }
    }
    fn swim(&mut self, mut i: usize) {
        while i > 0 && self.less((i - 1) / 2, i) {
            self.pq.swap((i - 1) / 2, i);
            i = (i - 1) / 2;
        }
    }

    fn less(&self, i: usize, j: usize) -> bool {
        self.pq[i] < self.pq[j]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut pq = MaxPQ::<usize>::new();
        assert_eq!(0, pq.len());
        assert_eq!(None, pq.pop());
    }

    #[test]
    fn one() {
        let mut pq = MaxPQ::<usize>::new();
        pq.push(1);
        assert_eq!(1, pq.len());
        assert_eq!(Some(1), pq.pop());
        assert_eq!(None, pq.pop());
    }

    #[test]
    fn one_string() {
        let mut pq = MaxPQ::<&str>::new();
        pq.push("abc");
        assert_eq!(1, pq.len());
        assert_eq!(Some("abc"), pq.pop());
        assert_eq!(None, pq.pop());
    }

    #[test]
    fn several_elements() {
        let mut pq = MaxPQ::<isize>::new();
        pq.push(1);
        pq.push(-1);
        pq.push(2);
        assert_eq!(3, pq.len());
        assert_eq!(Some(2), pq.pop());
        assert_eq!(Some(1), pq.pop());
        assert_eq!(Some(-1), pq.pop());
        assert_eq!(None, pq.pop());
    }

    #[test]
    fn push_and_pop() {
        let mut pq = MaxPQ::<isize>::new();
        pq.push(2);
        pq.push(-1);
        pq.push(4);
        assert_eq!(Some(4), pq.pop());
        pq.push(1);
        pq.push(3);
        assert_eq!(Some(3), pq.pop());
        assert_eq!(Some(2), pq.pop());
        assert_eq!(Some(1), pq.pop());
        assert_eq!(Some(-1), pq.pop());
        assert_eq!(None, pq.pop());
    }

    #[test]
    fn with_identical_keys() {
        let mut pq = MaxPQ::<isize>::new();
        pq.push(2);
        pq.push(2);
        pq.push(4);
        pq.push(4);
        pq.push(3);
        assert_eq!(Some(4), pq.pop());
        assert_eq!(Some(4), pq.pop());
        assert_eq!(Some(3), pq.pop());
        assert_eq!(Some(2), pq.pop());
        assert_eq!(Some(2), pq.pop());
        assert_eq!(None, pq.pop());
    }
}
