use super::base::*;

pub struct PreOrderIter<'a, G> {
	g: &'a G,
	v: usize,
	marked: Vec<bool>,
	stack: Vec<usize>,
}
impl<'a, G, E> Iterator for PreOrderIter<'a, G>
where
	E: Directed + 'a,
	G: Graph<Edge = E>,
{
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		if self.stack.is_empty() {
			while self.v < self.g.v_size() {
				if !self.marked[self.v] {
					self.stack.push(self.v);
					break;
				}
				self.v += 1;
			}
		}

		self.stack.pop().map(|v| {
			self.marked[v] = true;
			for e in self.g.adj(v) {
				if !self.marked[e.to()] {
					self.stack.push(e.to());
				}
			}

			v
		})
	}
}
pub trait PreOrder<G> {
	fn pre_order(&self) -> PreOrderIter<'_, G>;
}
impl<G, E> PreOrder<G> for G
where
	E: Directed,
	G: Graph<Edge = E>,
{
	fn pre_order(&self) -> PreOrderIter<'_, G> {
		PreOrderIter {
			g: self,
			v: 0,
			marked: vec![false; self.v_size()],
			stack: Vec::new(),
		}
	}
}
#[cfg(test)]
mod tests {
	use super::super::DiGraph;
	use super::*;

	#[test]
	fn empty() {
		let g = DiGraph::new(1);
		assert_eq!(vec![0], g.pre_order().collect::<Vec<_>>());
	}

	#[test]
	fn pre_order_simple() {
		let mut g = DiGraph::new(6);
		g.add_edge(0, 4);
		g.add_edge(4, 5);
		g.add_edge(4, 3);
		g.add_edge(3, 1);

		let r = g.pre_order().collect::<Vec<_>>();
		assert_eq!(6, r.len());
		if r[2] == 5 {
			assert_eq!(vec![0, 4, 5, 3, 1, 2], r);
		} else {
			assert_eq!(vec![0, 4, 3, 1, 5, 2], r);
		}
	}
}
