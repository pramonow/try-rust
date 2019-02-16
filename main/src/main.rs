mod adder;

//Hello world function
fn main() {
    println!("Hello, world!");
    // ⭐️ Function pointers, Usage as a Data Type
    let b = adder::plus_one;
    let c = b(5); //6

    let b: fn(i32) -> i32 = adder::plus_one; //same, with type inference
    let c = b(5); //6

    let b = adder::plus_one;
    fn plus_two(b: fn(i32) -> i32, x: i32) -> i32 {
        b(b(x))
    }
    
    println!("{}", plus_two(b, 2)); //4
}





