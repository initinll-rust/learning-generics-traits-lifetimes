// Lifetime Annotations in Function Signatures

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len(){
        x
    }
    else {
        y
    }
}

// Lifetime Annotations in Struct Definitions
#[derive(Debug)]
pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

// Lifetime Elision rules

/* 1. The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference.
In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); 
a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); 
and so on. */ 

/* 2. The second rule is that, if there is exactly one input lifetime parameter, 
that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32. */

/* 3. The third rule is that, if there are multiple input lifetime parameters, 
but one of them is &self or &mut self because this is a method, 
the lifetime of self is assigned to all output lifetime parameters. */

// applies 2 rule
impl<'a> ImportantExcerpt<'a> {
    pub fn level(&self) -> i32 {
        3
    }
}

// applies 3 rule
impl<'a> ImportantExcerpt<'a> {
    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Generic Type Parameters, Trait Bounds, and Lifetimes Together

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
where T : Display, {    
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}