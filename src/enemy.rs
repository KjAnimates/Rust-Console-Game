use crate::game::{Game};
pub struct Enemy<'a> {
    pub name: String,
    pub health: f32,
    pub dead: bool,

    _game: &'a Game,
} impl<'a> Enemy<'a> {
    pub fn new(name: String, game_instance: &Game) -> Enemy {
        return Enemy {
            name: name,
            health: 100.0,
            dead: false,
            _game: game_instance
        };
    }
    pub fn print_stats(&self) {
        println!("Stats for '{}':", self.name);
        println!("  | health    {}", self.health);
        println!("  | dead?     {}", self.dead);
    }
        
    pub fn damage(&mut self, amount: f32) {
        if self.health - amount <= 0.0 {
            self.dead = true;
            self.health = 0.0;
            return;
        }
        self.health -= amount;
    }

    pub fn heal(&mut self, amount: f32) {
        if self.health + amount > self._game.max_humanoid_health {
            self.health = self._game.max_humanoid_health;
            return;
        }
        self.health += amount;
    }

}

#[cfg(test)]
mod enemy_testing {
    use super::*;

    #[test]
    fn test_damage_works() {
        let g = Game::new();
        let mut e = Enemy::new("Tabul".to_string(), &g);

        e.damage(50.00);

        assert_eq!(e.health, 50.00);
    }

    #[test]
    fn test_damage_reset_below_zero() {
        let g = Game::new();

        let mut e = Enemy::new("Tabul".to_string(), &g);
        e.damage(500.00);

        assert_eq!(e.health, 0.00);
    }

    #[test]
    fn test_health_below_zero_kills_enemy() {
        let g = Game::new();

        let mut e = Enemy::new("Tabul".to_string(), &g);
        e.damage(500.00);

        assert_eq!(e.dead, true);
    }
}
