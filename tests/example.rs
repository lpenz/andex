// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use andex::*;
use std::convert::TryFrom;

// A player with score
#[derive(Default, Clone, Copy)]
pub struct Player {
    pub score: i32,
}

// The player identifier
pub enum PlayerIdMarker {}
type PlayerId = Andex<PlayerIdMarker, usize, 4>;

// All players in the game
type Players = AndexableArray<PlayerId, Player, { PlayerId::SIZE }>;

// A piece on the board
#[derive(Default, Clone, Copy)]
pub struct Piece {
    pub owner: PlayerId,
    pub position: i32,
}

// The piece identifier
pub enum PieceIdMarker {}
type PieceId = Andex<PieceIdMarker, usize, 32>;

// All pieces in the game
type Pieces = AndexableArray<PieceId, Piece, { PieceId::SIZE }>;

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
        self.pieces[PieceId::try_from(0).unwrap()].position += 1;
        // ^ note that we had to use a const generic parameter so that
        // the index bound is checked at compile time.
        // If we want to create an index at run time, we have to use
        // TryInto/TryFrom, which returns Result:
        if let Ok(playerid) = PlayerId::try_from(1) {
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
