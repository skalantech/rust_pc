use aggregator::{Summary, Tweet, NewsArticle, self, SummaryDefault, 
                    SummaryDefault1, notify, notify2};

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
}