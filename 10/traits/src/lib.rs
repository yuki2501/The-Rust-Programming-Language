use std::fmt::Display;
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y:T) -> Self{
        Self {x, y }
    }
}
impl<T: Display + PartialOrd> Pair<T>{
    fn cmp_display(&self){
        if self.x >= self.y {
            println!("The largest member is x = {}",self.x);
        } else {
            println!("The largest member is y = {}",self.y);
        }
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle{
    fn summarize(&self) -> String{
        format!("{}, by {} ({})",self.headline, self.author, self.location)
    }
}
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

pub trait Summary {
    fn summarize(&self) -> String{
        // "read more"
        String::from("(Read more ...)")
    }
}
pub fn notify(item: &impl Summary){
    println!("Breaking news! {}",item.summarize());
}
// pub fn notify <T: Summary>(item: &T){
// 速報！
//    println!("Breaking news! {}",  item.summarize());
// }
//
// fn some_function<T, U>(t: &T, u: &U) -> i32
//    where T: Display + Clone,
//          U: Clone + Debug
// {
//   
// }
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

