fn main() {
    let mut count = 1;
    let mut inc = || count += 1;

    inc();
    println!("{}", count);
}
