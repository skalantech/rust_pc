
#[derive(Debug)]
enum Option_i32 {
    Some(i32),
    None,
}

#[derive(Debug)]
enum Option_f64 {
    Some(f64),
    None,
}


fn main() {
    let integer = Option_i32::Some(5);
	let integer = match integer {
		Option_i32::Some(5) => Some(5),
		_=> None
	};
    let float = Option_f64::Some(5.0);
	let float = match float {
		Option_f64::Some(5.0) => Some(5.0),
		_=> None
	};
	println!("{:?}", integer.unwrap());
	println!("{:?}", float.unwrap());
}