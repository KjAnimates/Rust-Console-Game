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
        
    }

}
