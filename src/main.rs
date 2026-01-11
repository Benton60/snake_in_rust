use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;


#[derive(Clone, Copy, PartialEq)]
enum Direction {
	North,
	East,
	South,
	West,
}

struct Coord{
	x: usize,
	y: usize,
}

struct Board{
	direction: Direction,
	snake_body: Vec<Coord>,
	head: Coord,
	apple: Coord,
}


fn main(){
	let mut board = Board{
		direction: Direction::South,
		snake_body: Vec::new(),
		head: Coord{ x: 10, y: 10 },
		apple: Coord{ x: 14, y: 10},
	};
	board.snake_body.push(Coord{ x: board.head.x, y: board.head.y});
	
	while !game_over(&board) {
		for _i in 0..500000 {
			update_direction_from_input(&mut board);
		}
		print_board(&board);
		if move_snake(&mut board) {
			add_apple(&mut board);
		}else{
			board.snake_body.remove(0);
		}
	}
}











fn move_snake(board: &mut Board) -> bool {
	match board.direction {
		Direction::North => {
				board.snake_body.push(Coord{ x: board.head.x - 1, y: board.head.y});
				board.head.x -= 1;
				return board.head.x == board.apple.x && board.head.y == board.apple.y
			},
		Direction::East => {
				board.snake_body.push(Coord{ x: board.head.x, y: board.head.y + 1});
				board.head.y += 1;
				return board.head.x == board.apple.x && board.head.y == board.apple.y
			},
		Direction::South => {
				board.snake_body.push(Coord{ x: board.head.x + 1, y: board.head.y});
				board.head.x += 1;
				return board.head.x == board.apple.x && board.head.y == board.apple.y
			},
		Direction::West => {
				board.snake_body.push(Coord{ x: board.head.x, y: board.head.y - 1});
				board.head.y -= 1;
				return board.head.x == board.apple.x && board.head.y == board.apple.y
				
			},
	}
}




fn game_over(board: &Board) -> bool {
	for (i, coord) in board.snake_body.iter().enumerate() {

		//checks that the snake is in-bounds
		if coord.x >= 21 || coord.y >= 21 {
			return true;
		}



		//checking no two vector coords are the same
		let slice: &[Coord] = &board.snake_body[i+1..];
		for inner_coord in slice.iter(){
			if inner_coord.x == coord.x && inner_coord.y == coord.y {
				return true;
			}
		}
	}
	
	false
}



//Prints the board 
fn print_board(board: &Board){

	clear_screen();    
    for _i in 0..44 {print!("_");}
	println!();
	for i in 0..21 {
		print!("|");
		for j in 0..21 {
			if board.apple.x == i && board.apple.y == j {
				print!(" O");
			}else if is_part_of_snake(i, j, board){
				print!(" #");
			}else{
				print!("  ");
			}
		}
		println!("|");
	}
	for _i in 0..44 {print!("-");}
	println!("\n");
}



fn clear_screen(){
    print!("{}[2J", 27 as char);
	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}


fn is_part_of_snake(i: usize, j:  usize, board: &Board) -> bool{
	for coord in board.snake_body.iter() {
		if coord.x == i && coord.y == j {return true}
	}
	false
}

fn add_apple(board: &mut Board){
	let mut x: usize = rand::random_range(..21);
	let mut y: usize = rand::random_range(..21);
	while is_part_of_snake(x,y, board){
		x = rand::random_range(..21);
		y = rand::random_range(..21);
	}
	board.apple.x = x;
	board.apple.y = y;	
}

fn update_direction_from_input(board: &mut Board) {
    if event::poll(Duration::from_millis(0)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            let new_direction = match key_event.code {
                KeyCode::Up => Some(Direction::North),
                KeyCode::Right => Some(Direction::East),
                KeyCode::Down => Some(Direction::South),
                KeyCode::Left => Some(Direction::West),
                _ => None,
            };

            if let Some(dir) = new_direction {
                // Prevent reversing direction (optional but recommended)
                if !is_opposite(board.direction, dir) {
                    board.direction = dir;
                }
            }
        }
    }
}



fn is_opposite(current: Direction, new: Direction) -> bool {
    matches!(
        (current, new),
        (Direction::North, Direction::South)
            | (Direction::South, Direction::North)
            | (Direction::East, Direction::West)
            | (Direction::West, Direction::East)
    )
}


