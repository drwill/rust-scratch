#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let area = rect1.area();
    println!("The rectangle is {rect1:?}.");
    println!("The area of the rectangle is {} square pixels.", area);
}

