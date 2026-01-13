
pub mod all{
	pub mod ui {
		use crate::structures::all::{Board, Coord};
		use crate::helpers::all::snake::is_part_of_snake;
		
		pub fn print_board(board: &Board) {
		    clear_screen();
		    for _i in 0..44 {
		        print!("_");
		    }
		    println!();
		    for i in 0..21 {
		        print!("|");
		        for j in 0..21 {
		            if board.apple.x == i && board.apple.y == j {
		                print!(" O");
		            } else if is_part_of_snake(i, j, board) {
		                print!(" #");
		            } else {
		                print!("  ");
		            }
		        }
		        println!("|");
		    }
		    for _i in 0..44 {
		        print!("-");
		    }
		    println!("\n");
		}
		pub fn clear_screen(){
			print!("{}[2J", 27 as char);
			print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
		}

		pub fn game_over(board: &Board) -> bool {
		    for (i, coord) in board.snake_body.iter().enumerate() {
		        // Check bounds
		        if coord.x >= 21 || coord.y >= 21 {
		            return true;
		        }
		
		        // Check self-collision
		        let slice: &[Coord] = &board.snake_body[i + 1..];
		        for inner_coord in slice.iter() {
		            if inner_coord.x == coord.x && inner_coord.y == coord.y {
		                return true;
		            }
		        }
		    }
		
		    false
		}
		
	}




	
	pub mod snake {
		use crate::structures::all::{Board, Direction, Coord};
		use crossterm::event::{self, Event, KeyCode};
		use std::time::Duration;
		use rand::Rng; // bring in the Rng trait
		
		pub fn is_part_of_snake(i: usize, j: usize, board: &Board) -> bool {
		    for coord in board.snake_body.iter() {
		        if coord.x == i && coord.y == j {
		            return true;
		        }
		    }
		    false
		}
		pub fn update_direction_from_input(board: &mut Board) {
		    if event::poll(Duration::from_millis(0)).unwrap() && let Event::Key(key_event) = event::read().unwrap() {
		        let new_direction = match key_event.code {
		            KeyCode::Up => Some(Direction::North),
		            KeyCode::Right => Some(Direction::East),
		            KeyCode::Down => Some(Direction::South),
		            KeyCode::Left => Some(Direction::West),
		            _ => None,
		        };
		        
		        if let Some(dir) = new_direction && !is_opposite(board.direction, dir) {
		            board.direction = dir;
		        }
		    }
		}
		
		pub fn is_opposite(current: Direction, new: Direction) -> bool {
		    matches!(
		        (current, new),
		        (Direction::North, Direction::South)
		            | (Direction::South, Direction::North)
		            | (Direction::East, Direction::West)
		            | (Direction::West, Direction::East)
		    )
		}
		
		pub fn add_apple(board: &mut Board) {
		    let mut rng = rand::thread_rng(); // create RNG
		    let mut x: usize = rng.gen_range(0, 21); // rand 0.3/0.4 style
		    let mut y: usize = rng.gen_range(0, 21);
		    while is_part_of_snake(x, y, board) {
		        x = rng.gen_range(0, 21);
		        y = rng.gen_range(0, 21);
		    }
		    board.apple.x = x;
		    board.apple.y = y;
		}
		
		pub fn move_snake(board: &mut Board) -> bool {
		    match board.direction {
		        Direction::North => {
		            board.snake_body.push(Coord { x: board.head.x - 1, y: board.head.y });
		            board.head.x -= 1;
		            board.head.x == board.apple.x && board.head.y == board.apple.y
		        }
		        Direction::East => {
		            board.snake_body.push(Coord { x: board.head.x, y: board.head.y + 1 });
		            board.head.y += 1;
		            board.head.x == board.apple.x && board.head.y == board.apple.y
		        }
		        Direction::South => {
		            board.snake_body.push(Coord { x: board.head.x + 1, y: board.head.y });
		            board.head.x += 1;
		            board.head.x == board.apple.x && board.head.y == board.apple.y
		        }
		        Direction::West => {
		            board.snake_body.push(Coord { x: board.head.x, y: board.head.y - 1 });
		            board.head.y -= 1;
		            board.head.x == board.apple.x && board.head.y == board.apple.y
		        }
		    }
		}
	}
}
