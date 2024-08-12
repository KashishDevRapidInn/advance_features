// fn add_one(x: i32) -> i32 {
//     x + 1
// }

// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 { //fn is a function pointer
//     f(arg) + f(arg)
// }

// fn main() {
//     let answer = do_twice(add_one, 5);

//     println!("The answer is: {answer}");
// }

fn main(){
    let list_of_numbers = vec![1, 2, 3];

    // using closure
    let list_of_strings: Vec<String> = list_of_numbers
                                                    .iter()
                                                    .map(|i| i.to_string())
                                                    .collect();
    //using fn pointer
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
                                                    .iter()
                                                    .map(ToString::to_string)
                                                    .collect();

    
}

//fn pointer
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
// closure
    (0u32..20).map(|x| Status::Value(x)).collect();


   
