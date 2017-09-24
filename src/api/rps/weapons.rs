use std::cmp::{Ord, Ordering, PartialOrd};
use std::str::FromStr;
use std::fmt;
use std::error::Error;
use rand::{thread_rng, Rng};
/// Rock Paper Scissors
#[derive(PartialEq, Eq)]
pub enum Weapons {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub struct WeaponParseError {
    description: String,
}

impl fmt::Display for WeaponParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Valid options are Rock/Paper/Scissors")
    }
}

impl Error for WeaponParseError {
    fn description(&self) -> &str {
        &self.description
    }
}



impl Weapons {
    pub fn to_string(&self) -> String {
        match *self {
            Weapons::Rock => String::from("Rock"),
            Weapons::Paper => String::from("Paper"),
            Weapons::Scissors => String::from("Scissors"),
        }
    }
    pub fn rand_weapon() -> Self {
        match "rps".chars().nth(thread_rng().gen_range(0, 3)).unwrap() {
            'p' => Weapons::Paper,
            's' => Weapons::Scissors,
            _ => Weapons::Rock,
        }
    }
}

impl FromStr for Weapons {
    // TODO: Use a proper error
    type Err = WeaponParseError;
    fn from_str(weapon: &str) -> Result<Self, Self::Err> {
        match weapon.to_lowercase().chars().nth(0) {
            Some(wep) => match wep {
                'r' => Ok(Weapons::Rock),
                'p' => Ok(Weapons::Paper),
                's' => Ok(Weapons::Scissors),
                _ => Err(Self::Err {
                    description: weapon.to_string(),
                }),
            },
            None => Err(Self::Err {
                description: weapon.to_string(),
            }),
        }
    }
}

impl PartialOrd for Weapons {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Weapons {
    fn cmp(&self, other: &Self) -> Ordering {
        match *self {
            Weapons::Rock => match *other {
                Weapons::Rock => Ordering::Equal,
                Weapons::Paper => Ordering::Less,
                Weapons::Scissors => Ordering::Greater,
            },
            Weapons::Paper => match *other {
                Weapons::Rock => Ordering::Greater,
                Weapons::Paper => Ordering::Equal,
                Weapons::Scissors => Ordering::Less,
            },
            Weapons::Scissors => match *other {
                Weapons::Rock => Ordering::Less,
                Weapons::Paper => Ordering::Greater,
                Weapons::Scissors => Ordering::Equal,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::Weapons;
    #[test]
    fn test_comparision() {
        assert!(Weapons::Rock == Weapons::Rock);
        assert!(Weapons::Rock > Weapons::Scissors);
        assert!(Weapons::Rock < Weapons::Paper);
        assert!(Weapons::Paper == Weapons::Paper);
        assert!(Weapons::Paper > Weapons::Rock);
        assert!(Weapons::Paper < Weapons::Scissors);
        assert!(Weapons::Scissors == Weapons::Scissors);
        assert!(Weapons::Scissors > Weapons::Paper);
        assert!(Weapons::Scissors < Weapons::Rock);
    }
}
