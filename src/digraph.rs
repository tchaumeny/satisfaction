use std::collections::HashSet;

use crate::cnf::{CNFFormula, CNFLiteral};


/// Determine satisfiability of a 2-SAT problem
/// 
/// This function will determine the satisfiability
/// of a CNF formula with clauses of size 2, using
/// the directed graph of implications for the formula,
/// as described in <https://cp-algorithms.com/graph/2SAT.html>
pub fn digraph_2sat(formula: &CNFFormula) -> bool {
    assert!(formula.clauses.iter().all(|clause| clause.literals.len() == 2));

    // Number of variables (actually an upper bound)
    let n = formula
        .clauses
        .iter()
        .flat_map(|clause| clause.literals.iter())
        .map(|literal| literal.variable)
        .max()
        .map_or_else(|| 0, |x| x + 1) as usize;

    let (adj, adj_t) = build_digraph(formula, 2 * n);

    // See <https://en.wikipedia.org/wiki/Kosaraju%27s_algorithm>
    let mut ordering: Vec<usize> = vec![];
    let mut visited: HashSet<usize> = HashSet::new();
    for vertex in 0..(2 * n) {
        dfs1(&adj, vertex, &mut visited, &mut ordering);
    }
    let mut components = vec![None::<usize>; 2 * n];
    for vertex in ordering.into_iter().rev() {
        dfs2(&adj_t, vertex, vertex, &mut components);
    }
    return (0..n).all(|i| components[2 * i] != components[2 * i + 1]);
}

#[inline(always)]
fn compute_idx(literal: &CNFLiteral, negate: bool) -> usize {
    if negate == literal.negate {
        2 * literal.variable as usize
    } else {
        (2 * literal.variable + 1) as usize
    }
}

fn build_digraph(formula: &CNFFormula, vertices: usize) -> (Vec<HashSet<usize>>, Vec<HashSet<usize>>) {
    let mut adj = vec![HashSet::new(); vertices];
    let mut adj_t = vec![HashSet::new(); vertices];

    for clause in formula.clauses.iter() {
        adj[compute_idx(&clause.literals[0], true)].insert(compute_idx(&clause.literals[1], false));
        adj_t[compute_idx(&clause.literals[1], false)].insert(compute_idx(&clause.literals[0], true));
        adj[compute_idx(&clause.literals[1], true)].insert(compute_idx(&clause.literals[0], false));
        adj_t[compute_idx(&clause.literals[0], false)].insert(compute_idx(&clause.literals[1], true));
    }

    return (adj, adj_t);
}

fn dfs1(adj: &Vec<HashSet<usize>>, vertex: usize, visited: &mut HashSet<usize>, ordering: &mut Vec<usize>) {
    if visited.contains(&vertex) {
        return;
    }
    visited.insert(vertex);
    for neighbour in adj[vertex].iter() {
        dfs1(adj, *neighbour, visited, ordering);
    }
    ordering.push(vertex);
}

fn dfs2(adj_t: &Vec<HashSet<usize>>, root: usize, vertex: usize, components: &mut Vec<Option<usize>>) {
    if components[vertex].is_some() {
        return;
    }
    components[vertex] = Some(root);
    for neighbour in adj_t[vertex].iter() {
        dfs2(adj_t, root, *neighbour, components);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cnf::CNFClause;
    use crate::rand_cnf::generate_cnf;

    #[test]
    fn test_simple_satisfiable() {
        let formula = CNFFormula {
            clauses: vec![
                CNFClause {
                    literals: vec![
                        CNFLiteral {
                            negate: false,
                            variable: 0,
                        },
                        CNFLiteral {
                            negate: true,
                            variable: 1,
                        },
                    ],
                },
                CNFClause {
                    literals: vec![
                        CNFLiteral {
                            negate: false,
                            variable: 1,
                        },
                        CNFLiteral {
                            negate: false,
                            variable: 2,
                        },
                    ],
                },
                CNFClause {
                    literals: vec![
                        CNFLiteral {
                            negate: true,
                            variable: 2,
                        },
                        CNFLiteral {
                            negate: true,
                            variable: 0,
                        },
                    ],
                },
            ],
        };
        assert!(digraph_2sat(&formula));
    }

    #[test]
    fn test_random_satisfiable() {
        let formula = generate_cnf(2, 25, 0.5, Some(42));
        assert!(digraph_2sat(&formula));
    }

    #[test]
    fn test_random_unsatisfiable() {
        let formula = generate_cnf(2, 25, 2., Some(42));
        assert!(!digraph_2sat(&formula));
    }
}
