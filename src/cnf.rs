use std::fmt;

#[derive(Clone)]
pub struct CNFLiteral {
    pub negate: bool,
    pub variable: u32,
}

impl fmt::Display for CNFLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}x_{{{}}}",
            if self.negate { "¬" } else { "" },
            self.variable
        )
    }
}

#[derive(Clone)]
pub struct CNFClause {
    pub literals: Vec<CNFLiteral>,
}

impl fmt::Display for CNFClause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.literals
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("∨")
        )
    }
}

#[derive(Clone)]
pub struct CNFFormula {
    pub clauses: Vec<CNFClause>,
}

impl fmt::Display for CNFFormula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.clauses
                .iter()
                .map(|x| format!("({x})"))
                .collect::<Vec<String>>()
                .join("∧")
        )
    }
}
