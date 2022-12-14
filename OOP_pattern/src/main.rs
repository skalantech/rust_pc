use OOP_pattern::Post;
use std::io;
use std::io::prelude::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    println!("{:?}", post.content());
    post.request_review();
    assert_eq!("", post.content());
    println!("{:?}", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("{:?}", post.content());
    pause();
}
