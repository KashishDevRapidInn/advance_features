/******************************/
//  Declarative macros
/******************************/


    // macro_rules! say_hello{
    //     () => {
    //         println!("Hello, world!");
    //     };
    // }

    // macro_rules! create_functions {
    //     ($($name:ident)*) => {
    //         $(
    //             fn $name() {
    //                 println!("Function {} called", stringify!($name));
    //             }
    //         )*
    //     };
    // }

    // create_functions!(foo bar baz);

    // // #[macro_export] // i did not use it because im not defining other file like lib.rs
    // macro_rules! vec {
    //     ( $( $x:expr ),* ) => {
    //         {
    //             let mut temp_vec = Vec::new();
    //             $(
    //                 temp_vec.push($x);
    //             )*
    //             temp_vec
    //         }
    //     };
    // }

    // fn main() {
    //     say_hello!(); 
    //     foo(); 
    //     bar(); 
    //     baz(); // Prints: Function baz called
    //     let v: Vec<u32> = vec![1, 2, 3];

    // }

/*************************************************************/
// Procedural Macros for Generating Code from Attributes
/*************************************************************/