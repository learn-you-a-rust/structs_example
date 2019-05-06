// this annotation allows us to print info about the rectangle
// with {:?}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// this is necessary if we want to have methods
impl Rectangle {
    // methods can take ownership of self or use mutable or 
    // immutable borrows of self; using &mut self allows the method
    // to change the instance it is called on
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // this is also a method
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // this is an associated function: it doesn't have the &self param;
    // String::from is another associated function (not relevant to this
    // struct);
    // these are useful to delineate a namespace
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    //let width1 = 30;
    //let height1 = 50;

    //let rect1 = (30, 50);
    let rect1 = Rectangle { width: 30, height: 50 };

    let rect2 = Rectangle { width: 10, height: 40 };
    
    let rect3 = Rectangle { width: 60, height: 45 };

    println!(
        "The area of the rectangle is {} square pixels.",
        //area(width1, height1)
        //area(rect1)
        //area(&rect1) // we now pass by reference so that main
                     // can continue referring to the rectangle
        rect1.area()
        );

    println!("rectl is {:#?}", rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // using an associated function;
    // this doesn't need to be associated with any instance of Rectangle 
    // to work, but instead is associated with the Rectangle struct itself
    let _sq = Rectangle::square(3);
}

// this parameter is an immutable borrow
//fn area(rectangle: &Rectangle) -> u32 {
//    rectangle.width * rectangle.height
//}

//fn area(dimensions: (u32, u32)) -> u32 {
//    dimensions.0 * dimensions.1
//}

//fn area(width: u32, height: u32) -> u32 {
//    width * height
//}
