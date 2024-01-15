use control_flow::control_flow;
use scalar_datatypes::{compound_types, scalar_types};
use variables::variables_main;

mod control_flow;
mod scalar_datatypes;
mod variables;

fn main() {
    variables_main();
    scalar_types();
    compound_types();
    control_flow()
}
