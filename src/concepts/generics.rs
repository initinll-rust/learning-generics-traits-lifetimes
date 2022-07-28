// non generic method for i32
pub fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// non generic method for char
pub fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// // generic method for T + Trait Bounds with where clause
pub fn largest_generic<T>(list: &[T]) -> T 
where T: Copy + std::cmp::PartialOrd {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generic struct with multiple Type - T & U
#[derive(Debug)]
pub struct PointA<T, U> {
    pub x: T,
    pub y: U,
}

// Generic struct with single Type - T
#[derive(Debug)]
pub struct PointB<T> {
    pub x: T,
    pub y: T,
}

// Generic struct implementation with single Type - T
impl<T> PointB<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

// Specific constrain of Type - f64
impl PointB<f64> {
    pub fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Generic struct with multiple Type - X1, Y1
#[derive(Debug)]
pub struct PointC<X1, Y1> {
    pub x: X1,
    pub y: Y1,
}

// Generic struct implementation with multiple Type - X1, Y1, X2, Y2 & multiple trait bounds
impl<X1, Y1> PointC<X1, Y1> {
    pub fn mixup<X2, Y2>(&self, other: &PointC<X2, Y2>) -> PointC<X1, Y2> 
    where 
        X1: Copy, 
        Y1: Copy, 
        X2: Copy, 
        Y2: Copy 
    {        
        PointC { x: self.x, y: other.y }
    }
}