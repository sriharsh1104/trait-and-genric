struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl<T> Point<T> {
    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
use std::io;
fn main() {
    let mut s = String::new();

    println!("ENTER THE VALUE => P");
    println!("Enter value of x :");
        
        io::stdin()
           .read_line(&mut s)
           .expect("failed to read line");

    let x = s.trim().parse().ok().expect("invalid output");

    let mut s = String::new();

    println!("Enter value of y :");
        
        io::stdin()
           .read_line(&mut s)
           .expect("failed to read line");

    let y = s.trim().parse().ok().expect("invalid output");

    let mut s = String::new();
    
    println!("ENTER THE VALUE OF => P1");
    println!("Enter value of x1 :");
        
        io::stdin()
           .read_line(&mut s)
           .expect("failed to read line");

    let x1 = s.trim().parse().ok().expect("invalid output");

    let mut s = String::new();

    println!("Enter value of y1 :");
        
        io::stdin()
           .read_line(&mut s)
           .expect("failed to read line");

    let y1 = s.trim().parse().ok().expect("invalid output");

    let p = Point { x: x, y: y };
    let p1 = Point { x: x1, y: y1};

    println!("p.x = {} p.y = {}", p.x(),p.y());
    println!("p1.x = {} p1.y = {}", p1.x(),p1.y());
    println!("{}",p.distance_from_origin());
    println!("{}",p1.distance_from_origin());

    let mut point = Vec::new();

    point.push(p.distance_from_origin());
    point.push(p1.distance_from_origin());

    let result= largest(&point);
    println!("Distance from origin {}",result);
    if p1.distance_from_origin() > p.distance_from_origin() {
        println!("Point 2 is far away from origin i.e. {}",p1.distance_from_origin())
    }
    else {
        println!("Point 1 is far away{}",p.distance_from_origin());
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}