use std::cmp::{Ord, Ordering};
use std::fmt::{self, Debug};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for Antigen {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "B" => Ok(Antigen::B),
            "AB" => Ok(Antigen::AB),
            "O" => Ok(Antigen::O),
            _ => Err(format!("Invalid antigen: {}", s)),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(format!("Invalid RH factor: {}", s)),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => self.rh_factor.cmp(&other.rh_factor),
            ord => ord,
        }
    }
}

impl FromStr for BloodType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err("Blood type string too short".to_string());
        }

        let (antigen_str, rh_str) = s.split_at(s.len() - 1);
        let antigen = Antigen::from_str(antigen_str)?;
        let rh_factor = RhFactor::from_str(rh_str)?;

        Ok(BloodType { antigen, rh_factor })
    }
}

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            match self.antigen {
                Antigen::A => "A",
                Antigen::B => "B",
                Antigen::AB => "AB",
                Antigen::O => "O",
            },
            match self.rh_factor {
                RhFactor::Positive => "+",
                RhFactor::Negative => "-",
            }
        )
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        // Check RH factor compatibility
        if matches!(self.rh_factor, RhFactor::Negative) && matches!(other.rh_factor, RhFactor::Positive) {
            return false;
        }

        // Check antigen compatibility
        match self.antigen {
            Antigen::O => other.antigen == Antigen::O,
            Antigen::A => matches!(other.antigen, Antigen::A | Antigen::O),
            Antigen::B => matches!(other.antigen, Antigen::B | Antigen::O),
            Antigen::AB => true,
        }
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut donors = Vec::new();
        let all_antigens = [Antigen::A, Antigen::B, Antigen::AB, Antigen::O];
        let all_rh_factors = [RhFactor::Positive, RhFactor::Negative];

        for antigen in all_antigens.iter() {
            for rh_factor in all_rh_factors.iter() {
                let blood_type = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh_factor.clone(),
                };
                if self.can_receive_from(&blood_type) {
                    donors.push(blood_type);
                }
            }
        }
        donors.sort();
        donors
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut recipients = Vec::new();
        let all_antigens = [Antigen::A, Antigen::B, Antigen::AB, Antigen::O];
        let all_rh_factors = [RhFactor::Positive, RhFactor::Negative];

        for antigen in all_antigens.iter() {
            for rh_factor in all_rh_factors.iter() {
                let blood_type = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh_factor.clone(),
                };
                if blood_type.can_receive_from(self) {
                    recipients.push(blood_type);
                }
            }
        }
        recipients.sort();
        recipients
    }
}
