/*************************************************************************/
// Creating Type Synonyms with Type Aliases
/*************************************************************************/
    // fn main(){
    //     type Kilometers = i32;

    //     let x: i32 = 5;
    //     let y: Kilometers = 5;

    //     println!("x + y = {}", x + y);


    //     // why use type alias
    //     // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    //     // fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    //     //     // --snip--
    //     // }

    //     // fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    //     //     // --snip--
    //     // }
    //     // as we can see we are using Box<dyn Fn() + Send + 'static> again and again(redundancy) we can define a type alias and write it as follows:
    //     type Thunk = Box<dyn Fn() + Send + 'static>;

    //     let f: Thunk = Box::new(|| println!("hi"));

    //     fn takes_long_type(f: Thunk) {
    //         // --snip--
    //     }

    //     fn returns_long_type() -> Thunk {
    //         // --snip--
    //     }

    // }


/*************************************************************************/
// The Never Type that Never Returns
/*************************************************************************/
// Syntax
// fn bar() -> ! {
    
// }
    // fn main(){
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue, // continue as a never type thats why we have given guess type as u32
    //     };

    // }
    // // The never type with the panic! macro 
    // impl<T> Option<T> {
    //     pub fn unwrap(self) -> T {
    //         match self {
    //             Some(val) => val,
    //             None => panic!("called `Option::unwrap()` on a `None` value"), // it has a return type of never
    //         }
    //     }
    // }
    // // infinite loops also have the never type


/*************************************************************************/
// Dynamically Sized Types and the Sized Trait
/*************************************************************************/

fn main(){
    //error
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";

    let s1: &str = "Hello there!";
    let s2: &str = "How's it going?";


}


//Non-Sized Types
fn print_length<T: ?Sized>(s: &T) {
    // This function can handle references to Sized or non-Sized types
    // For simplicity, we'll just print the size of the reference
    println!("Reference size: {}", std::mem::size_of_val(s));
}

fn main() {
    let my_str = "Hello";
    print_length(&my_str); // Works with &str
}