use rand::{rngs::StdRng, Rng, SeedableRng, RngCore};

use crate::cnf::{CNFClause, CNFFormula, CNFLiteral};


pub fn generate_cnf(k: u8, n: u32, alpha: f32, seed: Option<u64>) -> CNFFormula {
    assert!(n >= k.into());
    let mut rng: Box<dyn RngCore> = match seed {
        Some(val) => Box::new(StdRng::seed_from_u64(val)),
        None => Box::new(rand::thread_rng()),
    };
    let clauses_cnt = (alpha * (n as f32)) as usize;
    let clauses = (0..clauses_cnt)
        .map(|_| {
            let mut variables = vec![];
            while variables.len() < k.into() {
                let candidate = rng.gen_range(0..n);
                if !variables.contains(&candidate) {
                    variables.push(candidate);
                }
            }
            let literals = variables
                .into_iter()
                .map(|variable| CNFLiteral {
                    negate: rng.gen_bool(0.5),
                    variable,
                })
                .collect();
            CNFClause { literals: literals }
        })
        .collect();
    CNFFormula { clauses: clauses }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        let formula = generate_cnf(3, 5, 2., Some(42));
        assert_eq!(10, formula.clauses.len());
        assert!(formula.clauses.iter().all(|clause| clause.literals.len() == 3));
        assert_eq!(
            "(¬x_{1}∨¬x_{4}∨¬x_{3})∧(x_{4}∨x_{3}∨x_{2})∧(¬x_{3}∨¬x_{4}∨x_{2})∧(¬x_{2}∨¬x_{0}∨x_{3})∧(¬x_{2}∨x_{0}∨x_{1})∧(¬x_{2}∨x_{3}∨¬x_{4})∧(x_{4}∨¬x_{3}∨¬x_{1})∧(¬x_{0}∨¬x_{2}∨¬x_{3})∧(x_{1}∨x_{0}∨x_{2})∧(¬x_{4}∨x_{1}∨¬x_{3})",
            formula.to_string()
        );
    }
}
