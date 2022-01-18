use std::str::FromStr;

#[derive(Debug)]
struct Roll {
    n_dice: u16,
    n_sides: u16,
}

#[derive(Debug)]
struct BadRoll;

impl FromStr for Roll {
    type Err = BadRoll;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let specs: Vec<&str> = s.trim()
                                 .split('d')
                                 .collect();

        let n_sides = specs[0].parse::<u16>().or_else(|_| return Err(BadRoll))?;
        let n_dice = specs[1].parse::<u16>().or_else(|_| return Err(BadRoll))?;

        Ok(Roll { n_dice, n_sides })
    }
}


fn main() {
    // 6d1 -> roll 1x 6-sided die
    // 12d3 -> roll 3x 12-sided die

    // extension: add an extra method to string



    let roll: Roll = "6d1".parse().unwrap();//.roll();
    println!("{:?}", roll);

}