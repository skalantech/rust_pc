// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait SummaryDefault {
    fn summarizing(&self) -> String {
        String::from("(Read more...)")
    }
}

pub trait SummaryDefault1 {
    fn summarize_author(&self) -> String;

    fn summari_zing(&self) -> String {
        //String::from("(Read more...)")
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl SummaryDefault for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl SummaryDefault1 for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

pub fn notify2<T: SummaryDefault1>(item1: &T, item2: &T) {
    println!("Extra Extra! {}", item1.summari_zing());
    println!("Extra Extra! {}", item2.summari_zing());
}

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// Multiple trait bounds
// pub fn notify(item: &(impl Summary + Display)) {
// pub fn notify<T: Summary + Display>(item: &T) {

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

// Where clauses

// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
//     unimplemented!()
// }