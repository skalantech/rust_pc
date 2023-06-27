mod pair;

use aggregator::{Summary, Tweet, NewsArticle, self, SummaryDefault, 
                    SummaryDefault1, notify, notify2, returns_summarizable};

use std::fmt::Display;


struct Pair1<T> {
    x: T,
    y: T,
}

impl<T> Pair1<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair1<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    let tweet2 = Tweet {
        username: String::from("escacarlos"),
        content: String::from(
            "learning rust programming is fun",
        ),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("1 new tweet culo pelao: {}", tweet.summarize());
    println!("1 new tweet: {}", tweet.summari_zing());
    println!("New article available! {}", article.summarizing());
    println!("New article! {}", article.summarize());
    notify(&tweet, &article);
    notify2(&tweet, &tweet2);
    let tweet3 = returns_summarizable();
    println!("1 new tweet : {}", tweet3.summarize());

    let pair: Pair1<i32> = Pair1::new(20, 21);
    pair.cmp_display();

    let pair1: pair::Pair<f64> = pair::Pair::new(0.0015436, 0.00165478);
    pair1.cmp_display();

}