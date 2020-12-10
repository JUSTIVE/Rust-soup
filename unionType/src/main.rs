#[warn(non_snake_case)]
enum ShapeUnion{
    Circle{
        radius:f64
    },
    Rectangle{
        width:f64,
        height:f64
    }
}

fn get_size(shape:&ShapeUnion)-> f64 {
    match shape{
        ShapeUnion::Circle{radius} => radius * radius * std::f64::consts::PI,
        ShapeUnion::Rectangle{width,height} => width * height
    }
}

fn get_name(shape:&ShapeUnion) -> &'static str {
    match shape{
        ShapeUnion::Circle{radius:_} => "Circle",
        ShapeUnion::Rectangle{width:_,height:_} => "Rectangle"
    }
}

fn print_shape(shape:ShapeUnion) -> () {
    println!("{} {}",get_name(&shape),get_size(&shape))
}

fn main() {
    print_shape(ShapeUnion::Circle{radius:5.0});
    print_shape(ShapeUnion::Rectangle{width:5.0,height:3.0});
}
