mod enemy;
mod game;

use enemy::Enemy;
use game::Game;

fn main() {
    let instance = Game {
        max_humanoid_health: 240.00
    };

    let mut e = Enemy::new("Turbul".to_string(), &instance);
    e.print_stats();
    e.damage(10.00);
}
