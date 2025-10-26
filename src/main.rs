struct Point {
    x: f64,
    y: f64,
}

fn evaluate(p: &Point) {
    match p {
        Point{x:_, y:5.0} => println!("Point ends with 5"),
        Point{x:5.0, y:_} => println!("Point starts with 5"),
        Point{x,y} => {println!("Something else: {} {}", x, y)}
    }
}

fn main() {
    let p1: Point = Point{x:1.0,y:5.0};
    let p2: Point = Point{x:5.0,y:3.0};
    let p3: Point = Point{x:3.0,y:3.0};
    evaluate(&p1);
    evaluate(&p2);
    evaluate(&p3);
}