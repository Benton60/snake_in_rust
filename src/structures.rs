pub mod all {

	#[derive(Clone, Copy, PartialEq)]
	pub enum Direction {
	    North,
	    East,
	    South,
	    West,
	}
	
	
	pub struct Coord {
	    pub x: usize,
	    pub y: usize,
	}
	
	pub struct Board {
	    pub direction: Direction,
	    pub snake_body: Vec<Coord>,
	    pub head: Coord,
	    pub apple: Coord,
	}
}
