use std::fmt::Display;

use prettytable::{Row, Table};

pub fn print<T: Display>(t: Vec<Vec<T>>) {
    let mut table = Table::new();
    for r in t {
        table.add_row(Row::from(r));
    }
    table.printstd();
}
