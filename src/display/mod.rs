use ctrlc;
/// Helps with line wrap/unwrap and not leaving the user's terminal unwrapped.
/// When Start we unwrap, when Working we check for signals, when Stop we wrap.
pub enum RunState {
    Start,
    Working,
    Stop,
}
/// Displays a row using the specified character
pub fn display(v: &Vec<u8>, c: char, rs: RunState) {
    // Handle the line wrapping of the terminal.
    match rs {
        RunState::Start => {
            print!("\x1B[?7l");
            ctrlc::set_handler(move || {
                println!("Received signal TERM,INT or HUP. Exiting...");
                print!("\x1B[?7h");
                std::process::exit(1);
            })
            .expect("Error setting signal handler");
        }
        RunState::Stop => {
            print!("\x1B[?7h");
            return;
        }
        RunState::Working => {}
    }

    for i in v.into_iter() {
        if *i == 1 {
            print!("{}", c);
        } else {
            print!(" ")
        }
    }
    print!("\n");
}
