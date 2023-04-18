mod vec2;
mod animal;

use crate::vec2::Vec2;
use crate::animal::Animal;

fn vector_fun() {
    let a = Vec2 {
        x: 1,
        y: 2
    };

    println!("{:?}", a);

    let b = Vec2::<f32> {
        x: 3.0,
        y: 4.0
    };

    println!("{:?}", b);
    println!("{}", b.magnitude());
    
    let c = Vec2::<f32> {
        x: 1.0,
        y: 0.0
    };

    println!("{}", b.dot(c));
    println!("{:?}", c + b);
    println!("{:?}", c - b);
    println!("{:?}", -b);
}

fn animal_fun() {

}

fn main() {
    animal_fun();
}
