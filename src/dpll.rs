use itertools::Itertools;
use std::collections::HashSet;

use crate::cnf::{CNFClause, CNFFormula, CNFLiteral};

/// Determine satisfiability of a CNF formule
///
/// This function implements the DPLL algorithm as described
/// in <https://en.wikipedia.org/wiki/DPLL_algorithm#The_algorithm>
pub fn dpll_sat(init_formula: &CNFFormula) -> bool {
    let mut formula = init_formula.clone();

    // Unit propagation
    while let Some(literal) = formula
        .clauses
        .iter()
        .find(|clause| clause.literals.len() == 1)
        .map(|clause| &clause.literals[0])
    {
        formula = dpll_unit_propagate(&formula, literal)
    }

    // Pure litteral elimination
    let mut variables = HashSet::new();
    let mut pos_variables = HashSet::new();
    let mut neg_variables = HashSet::new();
    for literal in formula
        .clauses
        .to_vec()
        .into_iter()
        .flat_map(|clause| clause.literals)
    {
        variables.insert(literal.variable);
        if literal.negate {
            neg_variables.insert(literal.variable);
        } else {
            pos_variables.insert(literal.variable);
        }
    }

    for variable in pos_variables.difference(&neg_variables) {
        formula = dpll_pure_literal_elimination(
            &formula,
            &CNFLiteral {
                variable: *variable,
                negate: false,
            },
        );
    }
    for variable in neg_variables.difference(&pos_variables) {
        formula = dpll_pure_literal_elimination(
            &formula,
            &CNFLiteral {
                variable: *variable,
                negate: true,
            },
        );
    }

    // Stopping conditions
    if formula.clauses.is_empty() {
        return true;
    }
    if formula
        .clauses
        .iter()
        .any(|clause| clause.literals.is_empty())
    {
        return false;
    }

    // Recursive calls
    let variable = dpll_choose_literal(&formula);
    let mut clauses_pos = formula.clauses.to_vec();
    clauses_pos.push(CNFClause {
        literals: vec![CNFLiteral {
            variable,
            negate: false,
        }],
    });
    let mut clauses_neg = formula.clauses.to_vec();
    clauses_neg.push(CNFClause {
        literals: vec![CNFLiteral {
            variable,
            negate: true,
        }],
    });
    return dpll_sat(&CNFFormula {
        clauses: clauses_pos,
    }) || dpll_sat(&CNFFormula {
        clauses: clauses_neg,
    });
}

fn dpll_unit_propagate(formula: &CNFFormula, literal: &CNFLiteral) -> CNFFormula {
    let clauses = formula
        .clauses
        .iter()
        .map(|clause| {
            match clause
                .literals
                .iter()
                .find(|x| x.variable == literal.variable)
            {
                Some(&CNFLiteral { negate, .. }) => {
                    if negate == literal.negate {
                        None
                    } else {
                        Some(CNFClause {
                            literals: clause
                                .literals
                                .iter()
                                .filter(|x| x.variable != literal.variable)
                                .map(|x| x.clone())
                                .collect(),
                        })
                    }
                }
                None => Some(clause.clone()),
            }
        })
        .flatten()
        .collect();
    CNFFormula { clauses }
}

fn dpll_pure_literal_elimination(formula: &CNFFormula, literal: &CNFLiteral) -> CNFFormula {
    let clauses = formula
        .clauses
        .iter()
        .filter(|clause| {
            clause
                .literals
                .iter()
                .all(|x| x.variable != literal.variable)
        })
        .map(|clause| clause.clone())
        .collect();
    CNFFormula { clauses }
}

fn dpll_choose_literal(formula: &CNFFormula) -> u32 {
    return unsafe {
        formula
            .clauses
            .to_vec()
            .into_iter()
            .flat_map(|clause| clause.literals)
            .map(|literal| literal.variable)
            .sorted()
            .chunk_by(|&x| x)
            .into_iter()
            .map(|(key, chunk)| (key, chunk.count()))
            .max_by_key(|&(_, count)| count)
            .map(|(idx, _)| idx)
            .unwrap_unchecked()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_satisfiable() {
        let formula = CNFFormula {
            clauses: vec![
                CNFClause {
                    literals: vec![
                        CNFLiteral {
                            negate: false,
                            variable: 1,
                        },
                        CNFLiteral {
                            negate: true,
                            variable: 2,
                        },
                    ],
                },
                CNFClause {
                    literals: vec![
                        CNFLiteral {
                            negate: true,
                            variable: 1,
                        },
                        CNFLiteral {
                            negate: false,
                            variable: 2,
                        },
                        CNFLiteral {
                            negate: false,
                            variable: 3,
                        },
                    ],
                },
                CNFClause {
                    literals: vec![CNFLiteral {
                        negate: true,
                        variable: 1,
                    }],
                },
            ],
        };
        assert!(dpll_sat(&formula));
    }

    #[test]
    fn test_unsatisfiable() {
        let formula = CNFFormula {
            clauses: vec![
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
                        CNFLiteral {
                            negate: false,
                            variable: 3,
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
                        CNFLiteral {
                            negate: true,
                            variable: 3,
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
                            negate: true,
                            variable: 2,
                        },
                        CNFLiteral {
                            negate: false,
                            variable: 3,
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
                            negate: true,
                            variable: 2,
                        },
                        CNFLiteral {
                            negate: true,
                            variable: 3,
                        },
                    ],
                },
                CNFClause {
                    literals: vec![
                        CNFLiteral {
                            negate: true,
                            variable: 1,
                        },
                        CNFLiteral {
                            negate: false,
                            variable: 2,
                        },
                        CNFLiteral {
                            negate: false,
                            variable: 3,
                        },
                    ],
                },
                CNFClause {
                    literals: vec![
                        CNFLiteral {
                            negate: true,
                            variable: 1,
                        },
                        CNFLiteral {
                            negate: false,
                            variable: 2,
                        },
                        CNFLiteral {
                            negate: true,
                            variable: 3,
                        },
                    ],
                },
                CNFClause {
                    literals: vec![
                        CNFLiteral {
                            negate: true,
                            variable: 1,
                        },
                        CNFLiteral {
                            negate: true,
                            variable: 2,
                        },
                        CNFLiteral {
                            negate: false,
                            variable: 3,
                        },
                    ],
                },
                CNFClause {
                    literals: vec![
                        CNFLiteral {
                            negate: true,
                            variable: 1,
                        },
                        CNFLiteral {
                            negate: true,
                            variable: 2,
                        },
                        CNFLiteral {
                            negate: true,
                            variable: 3,
                        },
                    ],
                },
            ],
        };
        assert!(!dpll_sat(&formula));
    }
}
