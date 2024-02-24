fn main() {
    println!("Hello, world!");

    let num = &1;
    println!("{}", &num);

    // 字符串切片
    let str = "hello world";
    let str1 = &String::from("hello world");
    println!("{}", str);
    println!("{:p}", str);

    println!("\nEq PartialEq");
    let f1 = f32::NAN;
    let f2 = f32::NAN;
    if f1 == f2 {
        println!("f1 == f2");
    } else {
        println!("NAN 不能比较");
    }

    println!("\n切片");
    let str = "hello world";
    let num = [0; 4];
    println!("{}", str);
    println!("{:?}", num);
    let str = ["a"; 4];
    println!("{:?}", str);
}
