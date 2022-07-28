use std::mem::size_of_val;

pub fn type_conversion() {
    let x : u64 = 4279238655;
    let y = x as u32;

    println!("x: {}, sizeof x: {}", x, size_of_val(&x));
    println!("y: {}, sizeof y: {}", y, size_of_val(&y));

    if x == y as u64 {
        println!("eq");
    } else {
        println!("neq");
    }
}

/*
# Result

x: 4279238655, sizeof x: 8
y: 4279238655, sizeof y: 4
eq

*/