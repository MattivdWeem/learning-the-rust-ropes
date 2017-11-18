fn main() {

    let condition:bool = abstract_bool(false);
    let numb:u8 = if_in_let(condition);
    println!("condition was {}", condition);
    println!("The value of number is: {}", numb);


    let a = [10, 20, 30, 40, 50];

    // iterate over the elements in an array
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // do the same thing, with a generated list lengt, reversed
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // test string
    let mut s:String = String::from("hello");
    takes_ownership(&mut s);
    println!("{}",s);

}

fn takes_ownership(s: &mut String){
    s.push_str(", world");
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
