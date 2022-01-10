// Struct example using a rectangle
#[derive(Debug)] // Enables printing with {:?}
struct Rectangle {
    width: u32,
    height: u32,
}

// Implement a method to our rectangle struct
impl Rectangle {
    // Get area
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Get height
    fn height(&self) -> u32 {
        self.height
    }

    // See if given rect fits into this rect
    fn fits(&self, rect2: &Rectangle) -> bool {
        if self.area() >= rect2.area() {
            return true;
        }
        false
    }

    // Associated function, can't work with struct self params
    // This is a constructor for a square type of a rectangle
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size}
    }
}

fn main() {

    let rect = Rectangle { width: 40, height: 60};

    let area: u32 = rect_area(&rect);

    println!("Area of the rectangle: {}", area);

    // Defining how to print our rectangle variables
    println!("Our rectangle 'rect': {:?}", rect);
    // option #2, pretty print
    println!("\nOur rectangle 'rect': {:#?}", rect);
    // Getting the area from within the struct method
    println!("\nOur rectangle area: {}", rect.area());
    // Get only height of rectangle
    println!("H = {}", rect.height());

    // Test fits method
    let rectum = Rectangle { width: 600, height: 30};
    println!("Fits? > {}", rect.fits(&rectum));

    // Create a square using our implemented square associated function
    let my_sq = Rectangle::square(50);
    println!("Newly created rectangle {:#?}", my_sq);

}


fn rect_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}