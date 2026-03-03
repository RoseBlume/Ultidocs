mod rules;
use rules::{info, error, warning};

use crate::Linter;



pub fn linter() -> Linter {
    Linter::new()
        .add_rules(info::rules())
        .add_rules(error::rules())
        .add_rules(warning::rules())

}
