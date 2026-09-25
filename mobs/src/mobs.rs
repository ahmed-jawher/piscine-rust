pub mod boss;
pub mod member;

use std::collections::{HashMap, HashSet};

use boss::Boss;
use member::{Member, Role};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn recruit(&mut self, (name, age): (&str, u32)) {
        self.members
            .insert(name.to_string(), Member::new(Role::Associate, age));
    }

    pub fn attack(&mut self, target: &mut Mob) {
        let (winner, loser) = if self.power() > target.power() {
            (self, target)
        } else {
            (target, self)
        };

        if let Some(youngest) = loser.members.values().map(|m| m.age).min() {
            loser.members.retain(|_, m| m.age != youngest);
        }

        if loser.members.is_empty() {
            winner.cities.extend(loser.cities.drain());
            winner.wealth += loser.wealth;
            loser.wealth = 0;
        }
    }

    pub fn steal(&mut self, target: &mut Mob, value: u64) {
        let stolen = value.min(target.wealth);
        target.wealth -= stolen;
        self.wealth += stolen;
    }

    pub fn conquer_city(&mut self, mobs: &[&Mob], city: String) {
        if mobs.iter().all(|mob| !mob.cities.contains(&city)) {
            self.cities.insert(city);
        }
    }

    fn power(&self) -> u32 {
        self.members.values().map(|m| m.role.power()).sum()
    }
}
