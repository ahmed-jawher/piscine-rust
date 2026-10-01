#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16,
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> GameSession {
        GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
        }
    }

    pub fn read_winner(&self) -> Option<&(String, u16)> {
        if self.p1.1 > self.p2.1 {
            Some(&self.p1)
        } else if self.p2.1 > self.p1.1 {
            Some(&self.p2)
        } else {
            None
        }
    }

    pub fn update_score(&mut self, user_name: &str) {
        if self.is_game_finished() {
            return;
        }

        if user_name == self.p1.0 {
            self.p1.1 += 1;
        } else if user_name == self.p2.0 {
            self.p2.1 += 1;
        }
    }

    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }

    fn is_game_finished(&self) -> bool {
        let (p1_score, p2_score) = (self.p1.1, self.p2.1);

        p1_score + p2_score >= self.nb_games
            || p1_score * 2 > self.nb_games
            || p2_score * 2 > self.nb_games
    }
}
