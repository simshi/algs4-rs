use super::base::*;
use super::cycle::*;
use super::weighted_path::*;
use super::EdgeWeightedDirectedGraph;
use super::WeightedDirectedEdge;

use std::collections::VecDeque;

pub trait HasBellmanFordSP<E: Directed + Weighted> {
    fn bellmanford_sp(&self, s: usize) -> Result<WeightedPath<E>, Cycle>;
}
// general algorithm works for all valid graphs
impl<G, E> HasBellmanFordSP<E> for G
where
    E: Directed + Weighted,
    G: Graph<Edge = E>,
{
    fn bellmanford_sp(&self, s: usize) -> Result<WeightedPath<E>, Cycle> {
        let mut sp = WeightedPath::new(self.v_size(), std::f64::INFINITY);
        let mut b = BellmanfordSP::new(self.v_size());
        match b.detect(self, s, &mut sp) {
            Some(c) => Err(c),
            _ => Ok(sp),
        }
    }
}

struct BellmanfordSP {
    queue: VecDeque<Vertex>,
    on_queue: Vec<bool>,
    cost: usize,
}
impl BellmanfordSP {
    fn new(v: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            on_queue: vec![false; v],
            cost: 0,
        }
    }

    fn detect<G, E>(&mut self, g: &G, s: Vertex, sp: &mut WeightedPath<E>) -> Option<Cycle>
    where
        E: Directed + Weighted,
        G: Graph<Edge = E>,
    {
        self.queue.push_back(s);
        sp.dist_to[s] = 0.0;
        while let Some(v) = self.queue.pop_front() {
            self.on_queue[v] = false;
            let c = self.relax(g, v, sp);
            if c.is_some() {
                return c;
            }
        }

        None
    }
    fn relax<G, E>(&mut self, g: &G, v: Vertex, sp: &mut WeightedPath<E>) -> Option<Cycle>
    where
        E: Directed + Weighted,
        G: Graph<Edge = E>,
    {
        for e in g.adj(v) {
            let w = e.to();
            if sp.dist_to[w] > sp.dist_to[v] + e.weight() {
                sp.dist_to[w] = sp.dist_to[v] + e.weight();
                sp.edge_to[w] = Some(e);

                if !self.on_queue[w] {
                    self.queue.push_back(w);
                    self.on_queue[w] = true;
                }
            }
            self.cost += 1;
            if self.cost % g.v_size() == 0 {
                let c = Self::check_cycle(g, sp);
                if c.is_some() {
                    return c;
                }
            }
        }

        None
    }

    fn check_cycle<G, E>(g: &G, sp: &WeightedPath<E>) -> Option<Cycle>
    where
        E: Directed + Weighted,
        G: Graph<Edge = E>,
    {
        let mut cg = EdgeWeightedDirectedGraph::new(g.v_size());
        for e in sp.edge_to.iter().flatten() {
            cg.add_edge(&WeightedDirectedEdge::new(e.from(), e.to(), e.weight()));
        }

        CycleDetection::detect_directed(&cg)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::super::EdgeNonNegativeWeightedDirectedGraph as ENNWDG;
    use super::super::EdgeWeightedDAG;
    use super::super::EdgeWeightedDirectedGraph as EWDG;
    use super::super::NonNegativeWeightedDirectedEdge as NNWDE;
    use super::super::WeightedDirectedEdge as WDE;
    use super::super::WeightedDirectedEdge;
    use super::*;

    #[test]
    fn empty() {
        let g = EWDG::new(1);
        let r = g.bellmanford_sp(0);
        assert!(r.is_ok());
        let sp = r.ok().unwrap();
        assert_eq!(0, sp.dist_to(0).round() as usize);
    }

    #[test]
    fn one_edge() {
        let mut g = EWDG::new(3);
        g.add_edge(&WDE::new(0, 1, 1.0));

        let r = g.bellmanford_sp(0);
        assert!(r.is_ok());
        let sp = r.ok().unwrap();

        assert_eq!(None, sp.path_to(0).next());
        let a = sp.path_to(1).map(|e| e.to()).collect::<Vec<_>>();
        assert_eq!(vec![1], a);
        assert_eq!(None, sp.path_to(2).next());

        assert_eq!(0, sp.dist_to(0).round() as usize);
        assert_eq!(1, sp.dist_to(1).round() as usize);
        assert_eq!(f64::INFINITY, sp.dist_to(2));
    }

    #[test]
    fn tiny_ewd_with_negative_weights() {
        let ewd = vec![
            (4, 5, 0.35),
            (5, 4, 0.35),
            (4, 7, 0.37),
            (5, 7, 0.28),
            (7, 5, 0.28),
            (5, 1, 0.32),
            (0, 4, 0.38),
            (0, 2, 0.26),
            (7, 3, 0.39),
            (1, 3, 0.29),
            (2, 7, 0.34),
            (6, 2, -1.20),
            (3, 6, 0.52),
            (6, 0, -1.40),
            (6, 4, -1.25),
        ];
        let mut g = EWDG::new(8);
        for e in ewd {
            g.add_edge(&WDE::new(e.0, e.1, e.2));
        }
        let r = g.bellmanford_sp(0);
        assert!(r.is_ok());
        let sp = r.ok().unwrap();

        assert_eq!(None, sp.path_to(0).next());
        assert_eq!(0, (sp.dist_to(0) * 100.0).round() as usize);

        assert_eq!(
            vec![0, 2, 7, 3, 6, 4, 5],
            sp.path_to(1).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(93, (sp.dist_to(1) * 100.0).round() as usize);

        assert_eq!(vec![0], sp.path_to(2).map(|e| e.from()).collect::<Vec<_>>());
        assert_eq!(26, (sp.dist_to(2) * 100.0).round() as usize);

        assert_eq!(
            vec![0, 2, 7],
            sp.path_to(3).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(99, (sp.dist_to(3) * 100.0).round() as usize);

        assert_eq!(
            vec![0, 2, 7, 3, 6],
            sp.path_to(4).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(26, (sp.dist_to(4) * 100.0).round() as usize);

        assert_eq!(
            vec![0, 2, 7, 3, 6, 4],
            sp.path_to(5).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(61, (sp.dist_to(5) * 100.0).round() as usize);

        assert_eq!(
            vec![0, 2, 7, 3],
            sp.path_to(6).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(151, (sp.dist_to(6) * 100.0).round() as usize);

        assert_eq!(
            vec![0, 2],
            sp.path_to(7).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(60, (sp.dist_to(7) * 100.0).round() as usize);
    }

    #[test]
    fn tiny_ewd() {
        let ewd = vec![
            (4, 5, 0.35),
            (5, 4, 0.35),
            (4, 7, 0.37),
            (5, 7, 0.28),
            (7, 5, 0.28),
            (5, 1, 0.32),
            (0, 4, 0.38),
            (0, 2, 0.26),
            (7, 3, 0.39),
            (1, 3, 0.29),
            (2, 7, 0.34),
            (6, 2, 0.40),
            (3, 6, 0.52),
            (6, 0, 0.58),
            (6, 4, 0.93),
        ];
        let mut g = ENNWDG::new(8);
        for e in ewd {
            g.add_edge(&NNWDE::new(e.0, e.1, e.2).unwrap());
        }

        let r = g.bellmanford_sp(0);
        assert!(r.is_ok());
        let sp = r.ok().unwrap();

        assert_eq!(None, sp.path_to(0).next());
        assert_eq!(0, (sp.dist_to(0) * 100.0).round() as usize);

        assert_eq!(
            vec![0, 4, 5],
            sp.path_to(1).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(105, (sp.dist_to(1) * 100.0).round() as usize);

        assert_eq!(vec![0], sp.path_to(2).map(|e| e.from()).collect::<Vec<_>>());
        assert_eq!(26, (sp.dist_to(2) * 100.0).round() as usize);

        assert_eq!(
            vec![0, 2, 7],
            sp.path_to(3).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(99, (sp.dist_to(3) * 100.0).round() as usize);

        assert_eq!(vec![0], sp.path_to(4).map(|e| e.from()).collect::<Vec<_>>());
        assert_eq!(38, (sp.dist_to(4) * 100.0).round() as usize);

        assert_eq!(
            vec![0, 4],
            sp.path_to(5).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(73, (sp.dist_to(5) * 100.0).round() as usize);

        assert_eq!(
            vec![0, 2, 7, 3],
            sp.path_to(6).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(151, (sp.dist_to(6) * 100.0).round() as usize);

        assert_eq!(
            vec![0, 2],
            sp.path_to(7).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(60, (sp.dist_to(7) * 100.0).round() as usize);
    }

    #[test]
    fn tiny_ewdag() {
        let ewdag = vec![
            (5, 4, 0.35),
            (4, 7, 0.37),
            (5, 7, 0.28),
            (5, 1, 0.32),
            (4, 0, 0.38),
            (0, 2, 0.26),
            (3, 7, 0.39),
            (1, 3, 0.29),
            (7, 2, 0.34),
            (6, 2, 0.40),
            (3, 6, 0.52),
            (6, 0, 0.58),
            (6, 4, 0.93),
        ];
        let mut g = EWDG::new(8);
        for e in ewdag {
            g.add_edge(&WeightedDirectedEdge::new(e.0, e.1, e.2));
        }
        let ag = EdgeWeightedDAG::try_from(g).unwrap();

        let r = ag.bellmanford_sp(5);
        assert!(r.is_ok());
        let sp = r.ok().unwrap();

        assert_eq!(
            vec![5, 4],
            sp.path_to(0).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(73, (sp.dist_to(0) * 100.0).round() as usize);

        assert_eq!(vec![5], sp.path_to(1).map(|e| e.from()).collect::<Vec<_>>());
        assert_eq!(32, (sp.dist_to(1) * 100.0).round() as usize);

        assert_eq!(
            vec![5, 7],
            sp.path_to(2).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(62, (sp.dist_to(2) * 100.0).round() as usize);

        assert_eq!(
            vec![5, 1],
            sp.path_to(3).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(61, (sp.dist_to(3) * 100.0).round() as usize);

        assert_eq!(vec![5], sp.path_to(4).map(|e| e.from()).collect::<Vec<_>>());
        assert_eq!(35, (sp.dist_to(4) * 100.0).round() as usize);

        assert_eq!(None, sp.path_to(5).next());
        assert_eq!(0, (sp.dist_to(5) * 100.0).round() as usize);

        assert_eq!(
            vec![5, 1, 3],
            sp.path_to(6).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(113, (sp.dist_to(6) * 100.0).round() as usize);

        assert_eq!(vec![5], sp.path_to(7).map(|e| e.from()).collect::<Vec<_>>());
        assert_eq!(28, (sp.dist_to(7) * 100.0).round() as usize);
    }

    #[test]
    fn invalid_with_negative_cycle() {
        let ewd = vec![
            (0, 1, 0.35),
            (1, 2, 0.36),
            (2, 3, 0.37),
            (2, 4, 0.39),
            (4, 5, 0.40),
            (5, 2, -0.99),
        ];
        let mut g = EWDG::new(6);
        for e in ewd {
            g.add_edge(&WDE::new(e.0, e.1, e.2));
        }
        let r = g.bellmanford_sp(0);

        assert!(r.is_err());
        let c = r.err().unwrap();
        let a = c.iter().cloned().collect::<Vec<_>>();
        assert_eq!(vec![2, 4, 5, 2], a);
    }
}
