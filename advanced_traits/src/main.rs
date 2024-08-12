/*************************************************************************/
// Specifying Placeholder Types in Trait Definitions with Associated Types
/*************************************************************************/

    // pub trait Iterator {
    //     type Item; // associated type (Item is a placeholder)

    //     fn next(&mut self) -> Option<Self::Item>;
    // }
    // struct Counter{}

    // impl Iterator for COUNTER{
    //     type Item: u32;
    //     fn next(&mut self )-> Option<u32>{
    //         Some(0)
    //     }
    // }
    // Error: Because this we cannot implement a trait for a type more than once with different associated types. Each trait can be implemented for a type only once.

    // impl Iterator for COUNTER{
    //     type Item: u16;
    //     fn next(&mut self )-> Option<u16>{
    //         Some(0)
    //     }
    // }

    // Solution Generics(to have multiple implementations for a single trait )


    // pub Trait Iterator<T>{
    //     fn next(&mut self)-> Option<T>;
    // }

    // struct Counter{}
    // impl Iterator<u32> for COUNTER{
    //     fn next(&mut self )-> Option<u32>{
    //         Some(0)
    //     }
    // }
    // impl Iterator<u16> for COUNTER{
    //     fn next(&mut self )-> Option<u16>{
    //         Some(0)
    //     }
    // }


/*************************************************************************/
// Default Generic Type Parameters and Operator Overloading
/*************************************************************************/

    // Adding same type
        // use std::ops::Add;

        //     #[derive(Debug, PartialEq)]
        //     struct Point {
        //         x: i32,
        //         y: i32,
        //     }

        //     impl Add for Point {
        //         type Output = Point;

        //         fn add(self, other: Point) -> Point {
        //             Point {
        //                 x: self.x + other.x,
        //                 y: self.y + other.y,
        //             }
        //         }
        //     }

        //     fn main() {
        //         assert_eq!(
        //             Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        //             Point { x: 3, y: 3 }
        //         );
        //     }

    //Adding differnt types
        // use std::ops::Add;

        // struct Millimeters(u32);
        // struct Meters(u32);

        // impl Add<Meters> for Millimeters {
        //     type Output = Millimeters;

        //     fn add(self, other: Meters) -> Millimeters {
        //         Millimeters(self.0 + (other.0 * 1000))
        //         }
        //     }   
        //     fn main() {
        //     let mm = Millimeters(500); // 500 millimeters
        //     let m = Meters(2); // 2 meters

        //     let result = mm + m; // This will use the Add implementation

        //     println!("Result: {} millimeters", result.0);
        //     }   

/*************************************************************************/
// Calling Methods with the Same Name
/*************************************************************************/

    // trait Pilot {
    //     fn fly(&self);
    // }

    // trait Wizard {
    //     fn fly(&self);
    // }

    // struct Human;

    // impl Pilot for Human {
    //     fn fly(&self) {
    //         println!("This is your captain speaking.");
    //     }
    // }

    // impl Wizard for Human {
    //     fn fly(&self) {
    //         println!("Up!");
    //     }
    // }

    // impl Human {
    //     fn fly(&self) {
    //         println!("*waving arms furiously*");
    //     }
    // }
    // fn main() {
    //     let person = Human;
    //     Pilot::fly(&person);
    //     Wizard::fly(&person);
    //     person.fly();
    // }

/*************************************************************************/
//  Associated function of the same name
/*************************************************************************/

    // In associated fn we need to use Fully Qualified Syntax 
    // trait Pilot {
    //     fn fly(); // Associated function
    // }

    // trait Wizard {
    //     fn fly(); // Associated function
    // }

    // struct Human;

    // impl Pilot for Human {
    //     fn fly() {
    //         println!("This is your captain speaking.");
    //     }
    // }

    // impl Wizard for Human {
    //     fn fly() {
    //         println!("Up!");
    //     }
    // }

    // impl Human {
    //     fn fly() {
    //         println!("*waving arms furiously*");
    //     }
    // }

    // fn main() {
    //     // Call associated function from the `Pilot` trait
    //     <Human as Pilot>::fly(); 
        
    //     // Call associated function from the `Wizard` trait
    //     <Human as Wizard>::fly();
        
    //     // Call associated function directly from the `Human` type
    //     Human::fly();
    // }

// study again
/************************************************************************************/
//  Using Supertraits to Require One Trait’s Functionality Within Another Trait
/************************************************************************************/

    // use std::fmt;

    // trait OutlinePrint: fmt::Display {
    //     fn outline_print(&self) {
    //         let output = self.to_string();
    //         let len = output.len();
    //         println!("{}", "*".repeat(len + 4));
    //         println!("*{}*", " ".repeat(len + 2));
    //         println!("* {output} *");
    //         println!("*{}*", " ".repeat(len + 2));
    //         println!("{}", "*".repeat(len + 4));
    //     }
    // }
    // struct Point {
    //     x: i32,
    //     y: i32,
    // }
    // impl OutlinePrint for Point {}
    // impl fmt::Display for Point {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         write!(f, "({}, {})", self.x, self.y)
    //     }
    // }


/***************************************************************************/
// Using the Newtype Pattern to Implement External Traits on External Types
/***************************************************************************/
    use std::fmt;

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    fn main() {
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {w}");
    }