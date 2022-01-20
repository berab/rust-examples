use std::str::FromStr;
use rand::{self, Rng};


trait DndRoll {
    fn roll(&self) -> u32;
}

impl DndRoll for &str {
    fn roll(&self) -> u32 {
        1
    }
}

#[derive(Debug)]
struct Roll {
    n_dice: u16,
    n_sides: u16,
}

#[derive(Debug)]
struct BadRoll;

impl Roll {
    fn roll(&self) -> u32 {
        let mut rng = rand::thread_rng();
        (0..self.n_dice)
            .map(|_| {
                rng.gen_range(1..=self.n_sides) as u32
            })
            .sum()
    }
}

impl FromStr for Roll {
    type Err = BadRoll;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let specs: Vec<&str> = s.trim()
                                 .split('d')
                                 .collect();

        let n_sides = specs[0].parse::<u16>().or_else(|_| Err(BadRoll))?;
        let n_dice = specs[1].parse::<u16>().or_else(|_| Err(BadRoll))?;

        Ok(Roll {n_dice, n_sides })
    }
}


fn main() {
    // 6d1 -> roll 1x 6-sided die
    // 12d3 -> roll 3x 12-sided die

    // extension: add an extra method to string


    // let total: Roll = "12d3".parse().unwrap();
    let total: u32 = "12d3".parse::<Roll>().unwrap().roll();
    let traited = "fssafsfad".roll();
    println!("{:?}", total);

}