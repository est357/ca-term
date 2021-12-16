/// Displays a row using the specified character
pub fn display(v: &Vec<u8>, c: char) {
    for i in v.into_iter() {
        if *i == 1 {
            print!("{}", c);
        } else {
            print!(" ")
        }
    }
    print!("\n");
}
