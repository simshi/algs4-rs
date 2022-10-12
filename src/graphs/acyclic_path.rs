use super::base::*;
use super::weighted_path::*;

pub trait HasAcyclicPath<E>
where
    E: Directed + Weighted,
{
    fn acyclic_sp(&self, s: usize) -> WeightedPath<E>;
    fn acyclic_lp(&self, s: usize) -> WeightedPath<E>;
}
impl<G, E> HasAcyclicPath<E> for G
where
    E: Directed + Weighted,
    G: Acyclic<Edge = E>,
{
    fn acyclic_sp(&self, s: usize) -> WeightedPath<E> {
        let mut p = WeightedPath::new(self.v_size(), std::f64::INFINITY);
        acyclic_sp(&mut p, self, s);
        p
    }

    fn acyclic_lp(&self, s: usize) -> WeightedPath<E> {
        let mut p = WeightedPath::new(self.v_size(), std::f64::NEG_INFINITY);
        acyclic_lp(&mut p, self, s);
        p
    }
}

fn acyclic_sp<G, E>(p: &mut WeightedPath<E>, g: &G, s: usize)
where
    E: Directed + Weighted,
    G: Acyclic<Edge = E>,
{
    p.dist_to[s] = 0.0;
    for v in g.topo_order() {
        for e in g.adj(v) {
            acyclic_sp_relax(p, e);
        }
    }
}
fn acyclic_sp_relax<E>(p: &mut WeightedPath<E>, e: E)
where
    E: Directed + Weighted,
{
    let v = e.from();
    let w = e.to();
    if p.dist_to[v] + e.weight() < p.dist_to[w] {
        p.dist_to[w] = p.dist_to[v] + e.weight();
        p.edge_to[w] = Some(e);
    }
}

fn acyclic_lp<G, E>(p: &mut WeightedPath<E>, g: &G, s: usize)
where
    E: Directed + Weighted,
    G: Acyclic<Edge = E>,
{
    p.dist_to[s] = 0.0;
    for v in g.topo_order() {
        for e in g.adj(v) {
            acyclic_lp_relax(p, e);
        }
    }
}
fn acyclic_lp_relax<E>(p: &mut WeightedPath<E>, e: E)
where
    E: Directed + Weighted,
{
    let v = e.from();
    let w = e.to();
    if p.dist_to[v] + e.weight() > p.dist_to[w] {
        p.dist_to[w] = p.dist_to[v] + e.weight();
        p.edge_to[w] = Some(e);
    }
}

#[cfg(test)]
mod tests {
    use super::super::EdgeWeightedDAG;
    use super::super::EdgeWeightedDirectedGraph as EWDG;
    use super::super::WeightedDirectedEdge;
    use super::*;

    #[test]
    fn empty_sp() {
        let g = EWDG::new(1);
        let ag = EdgeWeightedDAG::try_from(g).unwrap();

        let sp = ag.acyclic_sp(0);
        assert_eq!(0, sp.dist_to(0).round() as usize);

        let lp = ag.acyclic_lp(0);
        assert_eq!(0, lp.dist_to(0).round() as usize);
    }

    #[test]
    fn one_edge() {
        let mut g = EWDG::new(3);
        g.add_edge(&WeightedDirectedEdge::new(0, 1, 1.0));

        let ag = EdgeWeightedDAG::try_from(g).unwrap();
        let sp = ag.acyclic_sp(0);

        assert_eq!(None, sp.path_to(0).next());
        let a = sp.path_to(1).map(|e| e.to()).collect::<Vec<_>>();
        assert_eq!(vec![1], a);
        assert_eq!(None, sp.path_to(2).next());

        assert_eq!(0, sp.dist_to(0).round() as usize);
        assert_eq!(1, sp.dist_to(1).round() as usize);
        assert_eq!(f64::INFINITY, sp.dist_to(2));

        let lp = ag.acyclic_lp(0);
        let a = lp.path_to(1).map(|e| e.to()).collect::<Vec<_>>();
        assert_eq!(vec![1], a);
        assert_eq!(None, lp.path_to(2).next());

        assert_eq!(0, lp.dist_to(0).round() as usize);
        assert_eq!(1, lp.dist_to(1).round() as usize);
        assert!(!lp.has_path_to(2));
    }

    #[test]
    fn tiny_ewdag_sp() {
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

        let ag = EdgeWeightedDAG::try_from(g);

        let sp = ag.unwrap().acyclic_sp(5);
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
    fn tiny_ewdag_lp() {
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

        let ag = EdgeWeightedDAG::try_from(g);

        let lp = ag.unwrap().acyclic_lp(5);
        assert_eq!(
            vec![5, 1, 3, 6, 4],
            lp.path_to(0).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(244, (lp.dist_to(0) * 100.0).round() as usize);

        assert_eq!(vec![5], lp.path_to(1).map(|e| e.from()).collect::<Vec<_>>());
        assert_eq!(32, (lp.dist_to(1) * 100.0).round() as usize);

        assert_eq!(
            vec![5, 1, 3, 6, 4, 7],
            lp.path_to(2).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(277, (lp.dist_to(2) * 100.0).round() as usize);

        assert_eq!(
            vec![5, 1],
            lp.path_to(3).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(61, (lp.dist_to(3) * 100.0).round() as usize);

        assert_eq!(
            vec![5, 1, 3, 6],
            lp.path_to(4).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(206, (lp.dist_to(4) * 100.0).round() as usize);

        assert_eq!(None, lp.path_to(5).next());
        assert_eq!(0, (lp.dist_to(5) * 100.0).round() as usize);

        assert_eq!(
            vec![5, 1, 3],
            lp.path_to(6).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(113, (lp.dist_to(6) * 100.0).round() as usize);

        assert_eq!(
            vec![5, 1, 3, 6, 4],
            lp.path_to(7).map(|e| e.from()).collect::<Vec<_>>()
        );
        assert_eq!(243, (lp.dist_to(7) * 100.0).round() as usize);
    }
}
