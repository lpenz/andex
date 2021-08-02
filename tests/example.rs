// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use andex::*;
use std::convert::TryFrom;

// A player with score
#[derive(Default)]
pub struct Player {
    pub score: i32,
}

// All players in the game
#[derive(Default)]
pub struct Players([Player; 4]);

// The player identifier
type PlayerId = Andex<4>;

// Make Players[PlayerId] work
impl_andex_for!(Players, Player, PlayerId);

// A piece on the board
#[derive(Default)]
pub struct Piece {
    pub owner: PlayerId,
    pub position: i32,
}

// All pieces in the game
#[derive(Default)]
pub struct Pieces([Piece; 32]);

// The player identifier
type PieceId = Andex<32>;

// Make Pieces[PieceId] work
impl_andex_for!(Pieces, Piece, PieceId);

#[derive(Default)]
pub struct Game {
    pub players: Players,
    pub pieces: Pieces,
}

impl Game {
    pub fn play(&mut self) {
        // Increment all scores
        for playerid in PlayerId::iter() {
            self.players[playerid].score += 1;
        }
        // Move first piece forward:
        self.pieces[PieceId::new::<0>()].position += 1;
        // ^ note that we had to use a const generic parameter so that
        // the index bound is checked at compile time.
        // If we want to create an index at run time, we have to use
        // TryInto/TryFrom, which returns Result:
        if let Ok(playerid) = PlayerId::try_from(8) {
            self.players[playerid].score = 9;
        }
    }
}

#[test]
fn example_test() {
    let mut game = Game::default();
    game.play();
    for playerid in PlayerId::iter() {
        println!(
            "playerid display {}, debug {:?}; score {}",
            playerid, playerid, game.players[playerid].score
        );
    }
}
