pub mod structures;
pub mod helpers;

use structures::all::Board;
use structures::all::Coord;
use structures::all::Direction;
use crate::helpers::all::{ui::{print_board, game_over}, snake::{move_snake,add_apple, update_direction_from_input}};





fn main() {
    let mut board = Board {
        direction: Direction::South,
        snake_body: Vec::new(),
        head: Coord { x: 10, y: 10 },
        apple: Coord { x: 14, y: 10 },
    };
    board.snake_body.push(Coord {
        x: board.head.x,
        y: board.head.y,
    });

    while !game_over(&board) {
        for _i in 0..500_000 {
            update_direction_from_input(&mut board);
        }

        print_board(&board);

        if move_snake(&mut board) {
            add_apple(&mut board);
        } else {
            board.snake_body.remove(0);
        }
    }
}



