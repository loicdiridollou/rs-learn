use scalar_datatypes::{compound_types, scalar_types};
use variables::variables_main;

mod scalar_datatypes;
mod variables;

fn main() {
    variables_main();
    scalar_types();
    compound_types();
}
