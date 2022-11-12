use core::fmt;
use std::{
    io::{Read, Stdout, Write},
    u8,
};

use crate::render::Render;

use super::piece::{KnightMove, Piece};

#[derive(Clone, Copy)]
pub struct Square(pub Option<Piece>);

#[derive(Debug)]
enum TurnError {
    BadParse,
    InvalidMoveInput,
    IllegalMove,
}

pub struct GameState {
    pub squares: [Square; 64],
    pub turn_count: usize,
    pub is_complete: bool,
    pub player_loc: usize,
}

impl GameState {
    pub fn init() -> Self {
        let starting_square: usize = 7;
        let mut gs = GameState {
            squares: [Square(None); 64],
            turn_count: 0,
            is_complete: false,
            player_loc: starting_square,
        };
        gs.squares[starting_square] = Square(Some(Piece::Knight));
        gs.squares[27] = Square(Some(Piece::Queen));
        gs
    }

    pub fn main_loop(&mut self) -> std::io::Result<()> {
        let mut output = std::io::stdout();
        let mut input = std::io::stdin()
            .bytes()
            .filter(|b| b.is_ok_and(|&v| (v as char) != '\n'));
        self.render(&mut output)?;
        while !self.is_complete {
            let turn_result = GameState::get_move(input.next())
                .and_then(|mv| self.turn(mv))
                .and_then(|_| Ok(self.render(&mut output)));
            if turn_result.is_err() {
                println!("{:?}", turn_result.unwrap_err());
            }
        }
        Ok(())
    }

    fn turn(&mut self, mv: KnightMove) -> Result<(), TurnError> {
        let old_loc_coords = GameState::index_to_coords(self.player_loc);
        let new_loc =
            GameState::coord_to_index(GameState::apply_move(old_loc_coords, mv.coord_delta()))?;
        self.squares[self.player_loc] = Square(None);
        self.squares[new_loc] = Square(Some(Piece::Knight));
        self.player_loc = new_loc;
        Ok(())
    }

    fn get_move(character_byte: Option<std::io::Result<u8>>) -> Result<KnightMove, TurnError> {
        let character = character_byte.and_then(Result::ok).map(|byte| byte as char);
        match character {
            Some('o') => Ok(KnightMove::NNE),
            Some('p') => Ok(KnightMove::ENE),
            Some(';') => Ok(KnightMove::ESE),
            Some('l') => Ok(KnightMove::SSE),
            Some('k') => Ok(KnightMove::SSW),
            Some('j') => Ok(KnightMove::WSW),
            Some('u') => Ok(KnightMove::WNW),
            Some('i') => Ok(KnightMove::NNW),
            Some(_) => Err(TurnError::InvalidMoveInput),
            None => Err(TurnError::BadParse),
        }
    }

    fn coord_to_index(input: (i8, i8)) -> Result<usize, TurnError> {
        GameState::coord_in_bounds(input)?;

        let (x, y) = input;
        Ok((y * 8 + x) as usize)
    }

    fn coord_in_bounds(coords: (i8, i8)) -> Result<(), TurnError> {
        match coords {
            (0..=7, 0..=7) => Ok(()),
            _ => Err(TurnError::IllegalMove),
        }
    }

    fn index_to_coords(input: usize) -> (i8, i8) {
        assert!(input < 64);
        return ((input % 8) as i8, (input / 8) as i8);
    }

    fn apply_move((x, y): (i8, i8), (xd, yd): (i8, i8)) -> (i8, i8) {
        (x + xd, y + yd)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(Piece::Knight) => write!(f, "n "),
            Some(Piece::Queen) => write!(f, "q "),
            None => write!(f, ""),
        }
    }
}

impl Render<&mut Stdout> for GameState {
    fn render(&self, output: &mut Stdout) -> std::io::Result<()> {
        let mut board_str: String = String::new();
        for (i, square) in self.squares.iter().enumerate() {
            match square.0 {
                Some(_) => board_str.push_str(square.to_string().as_str()),
                None => board_str.push_str(if i % 2 ^ (i / 8) % 2 == 0 {
                    "\u{2591}\u{2591}"
                } else {
                    "\u{2588}\u{2588}"
                }),
            };
            if i % 8 == 7 {
                board_str.push('\n');
            }
        }
        output.write_all(board_str.as_bytes())?;
        output.flush()
    }
}
