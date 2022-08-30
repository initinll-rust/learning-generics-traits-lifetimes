use std::fmt::{Display,Debug};

pub trait Summary {
    fn summarize_author(&self) -> String;

    // Default Implementations
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.location)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Traits as Parameters
pub fn notify1(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}


// Trait Bound Syntax - Variation 1
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax - Variation 2
pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

// Trait Bound Syntax - Variation 3
pub fn notify4<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

// Specifying Multiple Trait Bounds with the + Syntax - Variation 1
pub fn notify5(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

// Specifying Multiple Trait Bounds with the + Syntax - Variation 2
pub fn notify6<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Clearer Trait Bounds with where Clauses
fn some_function1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> () {    
}

fn some_function2<T, U>(t: &T, u: &U) -> ()
where T: Display + Clone, 
      U: Clone + Debug
{    
}

// Returning Types that Implement Traits
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