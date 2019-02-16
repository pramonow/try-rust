
//Function with arguments 
pub fn print_sum(a: i8, b: i8) {
    println!("sum is: {}", a + b);
}

//Returning
pub fn plus_one(a: i32) -> i32 {
    a + 1
    // There is no ending ; in the above line. It means this is an expression which equals to `return a+1;`
}

pub fn plus_two(a: i32) -> i32 {
    return a + 2; //return a+2 but bad practice, 
    //should use only on conditional returnes, except it's last expression 
}
