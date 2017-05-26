use snake::Snake;
use state::WorldState;


/// This is just a gimmick function atm. Later it should calculate vision values etc and return all
/// senses as a Vec<f64> in the range -1.0 to 1.0
#[allow(unused)]
pub fn get(snake: &Snake, world: &WorldState) -> Vec<f64> {
    let sense = 1.0; // Range -1.0 to 1.0, should have unique senses later.
    let mut senses = Vec::new();
    for _ in 0..48 {
        senses.push(sense);
    }
    senses
}
