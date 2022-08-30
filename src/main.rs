mod concepts;

use concepts::generics::{largest_char, largest_generic, largest_i32, PointA, PointB, PointC};

use crate::concepts::traits::{Tweet, Summary, notify1, notify2, returns_summarizable};

fn main() {
    // ---------------------------------------------------------------------------
    // Generics

    let num1_list = vec![1, 2, 3, 4, 5];
    let char_list = vec!['y', 'm', 'a', 'q'];
    let num2_list = vec![10, 20, 30, 40, 50];

    let l_i32 = largest_i32(&num1_list);
    println!("Largest no - {}", l_i32);

    let l_char = largest_char(&char_list);
    println!("Largest char - {}", l_char);

    let l_generic = largest_generic(&num2_list);
    println!("Largest no - {}", l_generic);

    let both_integer = PointA { x: 5, y: 10 };
    println!("both_integer - {:?}", both_integer);

    let both_float = PointA { x: 1.0, y: 4.0 };
    println!("both_float - {:?}", both_float);

    let integer_and_float = PointA { x: 5, y: 4.0 };
    println!("integer_and_float - {:?}", integer_and_float);

    let point1 = PointB { x: 5, y: 10 };
    println!("point1 - {:?} {:?}", point1, point1.x());

    let point2 = PointB { x: 5.0, y: 10.0 };
    println!("point2 - {:?} {:?}", point2, point2.distance_from_origin());

    let point3 = PointC { x: 5.0, y: 10.0 };
    println!("point3 - {:?}", point3);

    let point4 = PointC { x: 5, y: 10 };
    println!("point4 - {:?}", point4);

    let point5 = point3.mixup(&point4);
    println!("point5 - {:?}", point5);

    println!("Again point3 - {:?} & point4 - {:?}", point3, point4);

    // Generics
    // ---------------------------------------------------------------------------
    // Traits

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet : {}", tweet.summarize());

    // Traits as Parameters
    notify1(&tweet);

    // Trait Bound Syntax
    notify2::<Tweet>(&tweet);

    let t = returns_summarizable();

    // Traits
    // ---------------------------------------------------------------------------
}
