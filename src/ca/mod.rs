use crate::display;
use std::{thread, time};

/// The automaton struct
pub struct CA {
    row: Vec<u8>,
    rule: u8,
    generations: u32,
    interval_between_generations: u64,
    display_character: char,
}

impl CA {
    ///  The constructor
    pub fn new(
        v: Vec<u8>,
        rule: u8,
        generations: u32,
        interval_between_generations: u64,
        display_character: char,
    ) -> CA {
        return CA {
            row: v,
            rule: rule,
            generations: generations,
            interval_between_generations: interval_between_generations,
            display_character: display_character,
        };
    }
    /// Generates the new rows after applying the rule and displays them
    pub fn generate(&mut self) {
        display::display(&self.row, self.display_character);
        for _ in 0..self.generations {
            let last_row = self.row.clone();
            self.row.clear();
            for (i, cur_val) in last_row.iter().enumerate() {
                if i == 0 || i == last_row.len() - 1 {
                    self.row.push(*cur_val);
                    continue;
                }
                let rez = self.check_rule(last_row[i - 1], *cur_val, last_row[i + 1]);

                self.row.push(rez);
            }

            display::display(&self.row, self.display_character);

            thread::sleep(time::Duration::from_micros(
                self.interval_between_generations,
            ));
        }
    }
    /// Returns the next value of a cell based on its neighbors and the rule
    fn check_rule(&self, left: u8, center: u8, right: u8) -> u8 {
        let mut checked_val = 0;
        checked_val = ((((checked_val | left) << 1) | center) << 1) | right;
        (self.rule >> checked_val) & 1
    }
}
