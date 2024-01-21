use control_flow::control_flow;
use ownership::ownership_main;
use scalar_datatypes::{compound_types, scalar_types};
use slices::slices_main;
use variables::variables_main;

mod control_flow;
mod ownership;
mod scalar_datatypes;
mod slices;
mod variables;

fn main() {
    variables_main();
    scalar_types();
    compound_types();
    control_flow();
    ownership_main();
    slices_main();
}
