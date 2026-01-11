
#[derive(Copy, Clone)]
enum Space {
	Blank,
	Player,
	Apple,
}

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
	board: [[Space; 21]; 21],	
	direction: Dirdection,
	snake_body: Vec<Coord>,
	head: Coord,
}


fn main(){
	let mut board = Board{
		board: [[Space::Blank;21]; 21],
		direction: Direction::South,
		snake_body: Vec::new(),
		head: Coord{ x: 10, y: 10 },
	};
	board.board[board.head.x][board.head.y] = Space::Player;
	
	
	
	
	while !game_over() {
		print_board(&board);
		
	}
}













fn game_over() -> bool {
	false
}



//Prints the board duh artard
fn print_board(board: &Board){

	clear_screen();    
    for _i in 0..44 {print!("_");}
	println!();
	for i in 0..21 {
		print!("|");
		for j in 0..21 {
			match board.board[i][j] {
				Space::Apple => print!(" O"),
				Space::Player => print!(" #"),
				_ => print!("  "),
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
