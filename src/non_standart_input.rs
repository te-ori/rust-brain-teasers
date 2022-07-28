use std::io::stdin;

pub fn non_standart_input() {
    println!("What is 3+2? Type your answer and press enter");

    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Unable to read standart input");
    
    // for c in input.chars() {
    //     println!("{}", c as i32);
    // }
    
    println!("{:#?}", input);
    println!("len: {}", input.len());

    if input == "5\r\n" {
        println!("Correct");
    } else {
        println!("In correct");
    }
}
