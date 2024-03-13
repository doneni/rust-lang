use std::io;

fn main() {
    let mut n = String::new();
    println!("n: ");
    io::stdin().read_line(&mut n)
        .expect("error");
    let n: i32 = n.trim().parse()
        .expect("n is integer");

    fib(n);
}

fn fib(n: i32) {
    let mut n_1 = 1;
    let mut n_2 = 1;
    for i in 1..n+1 {
        if i == 1 || i == 2 {
            println!("{}", 1);
        } else {
            println!("{}", n_1 + n_2);
            let tmp = n_1;
            n_1 = n_2;
            n_2 = tmp + n_2;
        }
    }
}
