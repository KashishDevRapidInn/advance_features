/**********************************/
//  Dereference a raw pointer
/**********************************/

    fn main(){
        let mut num= 5;
        let r1= &num as *const i32;
        let r2= &mut num as *const i32;

        // this is not possible in rust as it is not fllowing ownership rules: we cannot have a mutable and immutable refernce to same memory block at the same point
        // println!("{}, {}", *r1, *r2);
        unsafe{
            println!("{}, {}", *r1, *r2);
        }
        let address = 0x012345usize;
        let r = address as *const i32;
        unsafe {
            println!("{}", *r);
        }
    }

/**********************************/
//  Call an unsafe function or method
/**********************************/

    // fn main(){
    //     unsafe fn dangerous() {}

    //     unsafe {
    //         dangerous();
    //     }

    // }


/************************************************/
//  Creating a Safe Abstraction over Unsafe Code
/************************************************/

    // without unsafe rust as we can see w eare getting error on this line  (&mut values[..mid], &mut values[mid..]). because Rust’s borrow checker can’t understand that we’re borrowing different parts of the slice; it only knows that we’re borrowing from the same slice twice. Borrowing different parts of a slice is fundamentally okay because the two slices aren’t overlapping, but Rust isn’t smart enough to know this. When we know code is okay, but Rust doesn’t, it’s time to reach for unsafe code.


    // fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //     let len = values.len();
    //     assert!(mid <= len);
    //     (&mut values[..mid], &mut values[mid..])
    // }
    
    // use std::slice;


    // here we can see even though this fn contains unsafe block, we don't need to mark it as unsafe or call in unsafe blocks because we have created safe abstraction inside it

    // fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //     let len = values.len();
    //     let ptr = values.as_mut_ptr();

    //     assert!(mid <= len);

    //     unsafe {
    //         (
    //             slice::from_raw_parts_mut(ptr, mid),
    //             slice::from_raw_parts_mut(ptr.add(mid), len - mid),
    //         )
    //     }
    // }
    // fn main(){
    //     let mut v = vec![1, 2, 3, 4, 5, 6];
    //     let r = &mut v[..];
    //     let (a, b) = r.split_at_mut(3);
    //     assert_eq!(a, &mut [1, 2, 3]);
    //     assert_eq!(b, &mut [4, 5, 6]);

    // }

/***************************************************/
// Using extern Functions to Call External Code
/***************************************************/

    // Rust code might need to interact with code written in another language. For this, Rust has the keyword "extern" that facilitates the creation and use of a Foreign Function Interface (FFI).

    // Ex:  abs function from the C standard library
    //  "C" part defines which application binary interface (ABI) the external function uses: the ABI defines how to call the function at the assembly level.
    // extern "C" {
    //     fn abs(input: i32) -> i32;
    // }       

    // fn main() {
    //     unsafe {
    //         println!("Absolute value of -3 according to C: {}", abs(-3));
    //     }
    // }

    // we make the call_from_c function accessible from C code, after it’s compiled to a shared library and linked from C:
    // #[no_mangle]
    // pub extern "C" fn call_from_c() {
    //     println!("Just called a Rust function from C!");
    // }

/****************************************************/
//  Accessing or Modifying a Mutable Static Variable
/****************************************************/

    // static HELLO_WORLD: &str = "Hello, world!";

    // fn main() {
    //     println!("name is: {HELLO_WORLD}");
    // }

    // static mut COUNTER: u32 = 0;

    // fn add_to_count(inc: u32) {
    //     unsafe {
    //         COUNTER += inc;
    //     }
    // }

    // fn main() {
    //     add_to_count(3);

    //     unsafe {
    //         println!("COUNTER: {COUNTER}");
    //     }
    // }

/****************************************************/
//  Implementing an Unsafe Trait
/****************************************************/

    // unsafe trait Foo {
    //     // methods go here
    // }

    // unsafe impl Foo for i32 {
    //     // method implementations go here
    // }

    // fn main() {}

/****************************************************/
//  Accessing Fields of a Union
/****************************************************/

    // Define a union with two fields of different types
    // union MyUnion {
    //     int_field: i32,
    //     float_field: f32,
    // }

    // fn main() {
    //     // Initialize with only one field
    //     let mut my_union = MyUnion { int_field: 42 };

    //     unsafe {
    //         println!("int_field: {}", my_union.int_field);

    //         my_union.float_field = 3.14;

    //         println!("float_field: {}", my_union.float_field);

    //         // Note: Accessing int_field after setting float_field is unsafe and may lead to undefined behavior
    //         // println!("int_field: {}", my_union.int_field); // Uncommenting this may lead to incorrect results
    //     }
    // }
