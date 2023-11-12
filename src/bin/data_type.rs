fn main() {
  // boolean 
  let _first_bool: bool = true;

  let _second_bool: bool = false;

  // 8, 12, 32, 64, 128 bits
  // intergers can be either negative & zero & positive

  let _days_of_week: i8 =  7;

  let _number_of_users: i64 = 12800;

  // unsigned integers can be either 0 & positive
  // if there will be no negative values in variable,
  //we can use unsigned integers to save up half of the memory for the same jobs
  //PS: In smart contracts and blockchain there is no negative numbers in 

  let _number_of_toknes_v1 : i128 = 10000;
  // with using unsigned integer we can save up memory
  let _number_of_tokens_v2: u64= 10000;

  // 32,64 for float numbers

  let _pi : f32 = 3.14;   // if don't declare the f32 the default will be f64

  // Characters

  let _my_char: char = 'a';

  // String

  let _message: &str = "Hi, Dellenar"; // if we don't give type as &str it will be &str still by default


  let _my_string: String = String::from("Hi, Dellenar"); 

  // Arrays

  let _days_of_the_week: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
  ];

  let _first_element: &str = _days_of_the_week[0];

  let _last_element: &str = _days_of_the_week[_days_of_the_week.len() - 1];

  // Slices

  let _slice = &_days_of_the_week[1..3]; // "Teusday" & "Wednesday" is the values we get. the right value ('3') is exculuded

  let _first_element_slice = _slice[0];

  // Tuples: Tuples are immutable by default meaning that you can't modify the elements once Tuple has been created

  let _person = ("Dellenar", 24);

  let _name = _person.0;

  let _age = _person.1;

  // Unit type

  let _unit_type = ();

  // Variables

  let mut _num = 5;
  _num = 6;

}