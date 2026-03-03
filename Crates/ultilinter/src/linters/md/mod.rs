use crate::Linter;

mod rules;
use rules::{
    code,
    general,
    header,
    image,
    list,
    table
};

pub fn linter() -> Linter {
    Linter::new()
        .add_rules(code::rules())
        .add_rules(header::rules())
        .add_rules(general::rules())
        .add_rules(image::rules())
        .add_rules(list::rules())
        .add_rules(table::rules())
}


