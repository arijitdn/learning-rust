#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // methods
    fn area(&self) -> u32 {
        &self.width * &self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.height && self.height > other.height
    }
}

impl Rectangle {
    // associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };
    
    let rect2 = Rectangle {
        width: 6,
        height: 12
    };

    let rect3 = Rectangle {
        width: 50,
        height: 60
    };

    let rect3 = Rectangle::square(25);


    println!("Rectangle: {:#?}", rect);
    println!("Area is {} sq meteres", rect.area());

    println!("Rectangle can hold rect2: {}", rect.can_hold(&rect2));
    println!("Rectangle can hold rect3: {}", rect.can_hold(&rect3));
}