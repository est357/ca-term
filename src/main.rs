mod ca;
mod display;
mod process_args;

use crate::ca::CA;

fn main() {
    // Get cli args
    let (width, generations, rule, interval_between_generations, initial_bit, display_character) =
        process_args::process_args();

    // Initial state of the CA
    let mut starting_row = vec![0; width as usize];
    starting_row[initial_bit as usize] = 1;

    // Instantiate a new automaton based on used input.
    CA::new(
        starting_row,
        rule,
        generations,
        interval_between_generations,
        display_character,
    )
    .generate();
}
