use std::io;

fn main(){
    // let sum = 5 + 10;
    // let difference = 95.5 - 4.3;
    // let product = 4 *30 ;
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3;
    // let remainder = 43 % 5;
    // let t = true;
    // let f: bool = false;
    // let c = 'z';
    // let z: char = 'Z';
    // let heart_eyed_cat = '😻';
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // // let (x, y, a) = tup;
    // let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];
    // let _a: [i32; 5] = [ 1, 2, 3, 4, 5];
    // // let a = [ 1, 2, 3, 4, 5];
    // let a = [ 3; 5 ];
    // let mut b = a;
    // b[2] = 10;

    // println!("{sum}");
    // println!("{difference}");
    // println!("{product}");
    // println!("{quotient}");
    // println!("{truncated}");
    // println!("{remainder}");
    // println!("{t}");
    // println!("{f}");
    // println!("{c}");
    // println!("{z}");
    // println!("{heart_eyed_cat}");
    // println!("{0}", tup.0);
    // println!("{0}", tup.1);
    // println!("{0}", tup.2);
    // println!("{0}", a[0]);
    // println!("{0}", a[1]);
    // println!("{0}", months[0]);
    // println!("{0}", months[1]);
    // println!("{0}", months[2]);
    // println!("{0}", months[3]);
    // println!("{0}", months[4]);
    // println!("{0}", months[5]);
    // println!("{0}", months[6]);
    // println!("{0}", months[7]);
    // println!("{0}", months[8]);
    // println!("{0}", months[9]);
    // println!("{0}", months[10]);
    // println!("{0}", months[11]);
    // println!("{0}", a[0]);
    // println!("{0}", a[1]);
    // println!("{0}", a[2]);
    // println!("{0}", b[2]);
    // println!("{0}", a[3]);
    // println!("{0}", a[4]);
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}