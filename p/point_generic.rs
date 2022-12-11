
#[derive(Debug)]
struct Point<T> {
	x: T,
	y: T,
}

#[derive(Debug)]
struct Point2<T, U> {
	x: T,
	y: U,
}

fn main() {
	let integer = Point { x: 5, y: 10};
	let float = Point { x: 1.0, y: 4.0};
	let integer_float = Point2 { x: 6, y: 10.0};
	
	println!("integer: {:?}", integer);
	println!("float: {:?}", float);
	println!("integer_float: {:?}", integer_float);
}