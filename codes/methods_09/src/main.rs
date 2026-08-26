#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, another: &Rectangle) -> bool{
        self.width > another.width && self.height > another.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    
    let rect1: Rectangle = Rectangle{
        width: 30,
        height: 40,
    };

    let rect2: Rectangle = Rectangle{
        width: 20,
        height: 30,
    };

    let rect3: Rectangle = Rectangle::square(30);

    println!(
        "The area of rectangle is: {}",
        rect1.area()
    );

    println!(
        "Can rect2 fits in to rect 1? {}", 
        rect1.can_hold(&rect2)    
    );

    println!(
        "The area of rect3 is: {}",
        rect3.area()
    );

}
