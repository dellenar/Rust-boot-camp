fn main (){
    let sum: i32 = _add(1,3);
    println!("The sum is {}", sum);

    let day_of_the_week: &str = "Sunday";

    // if statment
    if day_of_the_week == "Sunday" {
        println!("It's race day")
    } else if day_of_the_week == "Saturday" {
        println!("It's qualify day")
    } else {
        println!("there is no race")
    }

    // while
    let mut counter: i32 = 0;

    while counter <= 5 {
        println!("Counter value is {}", counter);
        counter += 1;
    }

    // for loop
    for number in 1..5 {
        println!("Number is {}", number);
    }

    //loop
    counter = 0;
    loop{
        println!("Counter number is {}", counter);
        counter += 1;
        if counter == 6 {
            break;
        }
    }

    // match statment
    let num = 5;

    match num {
        1 => {
            println!("The number is one");
            println!("The is the first match arm");
        },
        2 => println!("The number is two"),
        3 => println!("The number is three"),
        4 => println!("The number is four"),
        5 => println!("The number is five"),
        _ => println!("The number is something else") // _ => for all the others numbers that possible to come like default option in switch case
    };

    let result = match num {
        1 => "The number is one",
        2 => "The number is two",
        3 => "The number is three",
        _ => "The number is something else",
    };

    println!("The result is {}", result)
}

fn _add(x:i32, y:i32) -> i32 {
 let result: i32 = x + y;
 return result;
}

fn _no_param(){
    println!("works");
    1;  // it's same as return 1;
}

