fn main() {

    let condition:bool = abstract_bool(false);
    let numb:u8 = if_in_let(condition);
    println!("condition was {}", condition);
    println!("The value of number is: {}", numb);

}

// revert a boolean
fn abstract_bool(condition:bool) -> bool {
    let response:bool = !condition;
    response
}

// check if the condition is true/false, based on the condition return 5 or 6
fn if_in_let(condition:bool) -> u8 {

    let number:u8 = if condition {
        5
    } else {
        6
    };

    number
}
