use std::collections::HashMap;

use super::BaseGraph;
use super::Graph;

pub struct SymbolGraph {
    st: HashMap<String, usize>,
    keys: Vec<String>,
    g: Graph,
}
impl SymbolGraph {
    pub fn new<T>(edges: T) -> Self
    where
        T: Iterator<Item = (String, String)>,
    {
        let mut st = HashMap::new();
        let mut keys = Vec::new();
        let mut indexed_edgeds = Vec::new();

        for (v, w) in edges {
            let vi = Self::get_or_add(&mut st, &mut keys, &v);
            let wi = Self::get_or_add(&mut st, &mut keys, &w);
            indexed_edgeds.push((vi, wi));
        }

        let size = st.len();
        let mut sg = SymbolGraph {
            st,
            keys,
            g: Graph::new(size),
        };
        for (v, w) in indexed_edgeds {
            sg.g.add_edge(v, w);
        }

        sg
    }

    pub fn v_size(&self) -> usize {
        self.g.v_size()
    }
    pub fn e_size(&self) -> usize {
        self.g.e_size()
    }
    pub fn contains(&self, key: &str) -> bool {
        self.st.get(key).is_some()
    }

    pub fn adj<'a>(&'a self, key: &str) -> Iter<'a> {
        let v = self.st.get(key).unwrap();
        Iter {
            iter: self.g.adj(*v).cloned().collect::<Vec<usize>>().into_iter(),
            keys: &self.keys,
        }
    }
}

// private methods
impl SymbolGraph {
    fn get_or_add(st: &mut HashMap<String, usize>, keys: &mut Vec<String>, key: &str) -> usize {
        match st.get(key) {
            None => {
                let i = keys.len();
                st.insert(key.to_string(), i);
                keys.push(key.to_string());
                i
            }
            Some(&v) => v,
        }
    }
}
pub struct Iter<'a> {
    iter: std::vec::IntoIter<usize>,
    keys: &'a Vec<String>,
}
impl<'a> Iterator for Iter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| &self.keys[x])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let edges = Vec::<(String, String)>::new();
        let sg = SymbolGraph::new(edges.into_iter());
        assert_eq!(0, sg.v_size());
        assert_eq!(0, sg.e_size());
    }

    #[test]
    fn basic_ops() {
        let edges = vec![
            (String::from("JFK"), String::from("MCO")),
            (String::from("JFK"), String::from("ATL")),
        ];
        let sg = SymbolGraph::new(edges.into_iter());
        assert_eq!(3, sg.v_size());
        assert_eq!(2, sg.e_size());

        let mut a1 = sg.adj(&String::from("JFK")).collect::<Vec<_>>();
        a1.sort_unstable();
        assert_eq!(vec![&String::from("ATL"), &String::from("MCO")], a1);

        assert_eq!(
            vec![&String::from("JFK")],
            sg.adj(&String::from("ATL")).collect::<Vec<_>>()
        );
        assert_eq!(
            vec![&String::from("JFK")],
            sg.adj(&String::from("MCO")).collect::<Vec<_>>()
        );
    }
}
