fn main() {
    
    if_else();
    loop_bucle();
    // loop_labels();
    while_bucle();
    for_bucle();
}

fn if_else(){
    let number = 3;

    if number != 0 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let condition = true;
    let number = if condition { 5 } else {6};

    println!("The value is: {number}");
}

fn loop_bucle(){
    let mut i: i32 = 0;
    loop {
        println!("again!");
        i = i+1;
        if i == 10 { break; };
    }
}

// fn loop_labels(){
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 { break; }
//             if remaining == 2 { break 'counting_up; }
//             remaining -= 1;
//         }
//         count += 1;
//     }
//     println!("End count = {count}");
// }

fn while_bucle() {

    let mut  i: i32 = 10;

    while i <= 10 {
        println!("{i}");
        i = i + 1;
    }
}

fn for_bucle(){
    let months = ["Enero", "Febrero", "Marzo", "Abril", "Mayo"];
    for month in months {
        println!("{month}");
    }
    for number in (1..4).rev(){
        println!("{number}!");
    }
    print!("LIFTOFF!!!");
}