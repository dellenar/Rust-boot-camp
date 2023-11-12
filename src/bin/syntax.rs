fn main() {
    let _message = "Hello World"; // String variable

    let _x: i32 = 42; // Number variable

    let _pi : f64 = 3.14; // float type

    let _is_rust_fun: bool = true; // boolean type

    let _letter_a: char = 'a'; // char type

    fn _add(x: i32, y:i32) -> i32 {
        return x + y; // or can be used without return like -> x + y  
    } // to create function do fn name()

    let x: i32 = 4;

    if x >= 4{
        println!("x is not negative");
    }else{
        println!("x is negative");
    }

    let mut i: i32  = 1;

    while i <= 5 {
        println!("{}", i);
        i += 1;
    }

}
