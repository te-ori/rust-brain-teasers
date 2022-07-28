pub fn compare_floats() {
    if 0.1 + 0.2 == 0.3 {
        println!("Arithmetic still works");
    } else {
        println!("Please rboot the universe");
    }
}

pub fn comapre_floats_with_diff() {
    println!("EPSILON: {:?}", f32::EPSILON);
    
    let diff = ((0.1f32 + 0.2f32) - 0.3f32).abs();
    println!("Diff: {:?}", diff);
    if diff < f32::EPSILON {
        println!("Arithmetic still works");
    }
}
