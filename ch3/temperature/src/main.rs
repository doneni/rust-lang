use std::io;

fn main() {
    println!("[1] f2c");
    println!("[2] c2f");
    let mut call = String::new();
    let mut temp = String::new();
    io::stdin().read_line(&mut call)
        .expect("select 1 or 2");
    let call: i32 = call.trim().parse()
        .expect("select 1 or 2");

    println!("temparature: ");
    io::stdin().read_line(&mut temp)
        .expect("type float");
    let temp: f64 = temp.trim().parse()
        .expect("type float");

    if call == 1 {
        f2c(temp);
    } else {
        f2c(temp);
    }
}

fn f2c(x: f64) {
    println!("c: {}", (x - 32.0) * (5.0 / 9.0))
}

fn c2f(x: f64) {
    println!("f: {}", x * 1.8 + 32.0)
}