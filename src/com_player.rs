use crate::game::*;

pub struct ComPlayer {}

impl ComPlayer {
    pub fn new() -> ComPlayer {
        ComPlayer {}
    }
    pub fn find_the_best_way(&self, game: &mut Game, peice: Peice) -> (usize, usize, [[i8; 8]; 8]) {
        let da_way = Game::the_point_if_you_can_place_and_effect(game.board, peice);
        da_way[0]
    }
}
