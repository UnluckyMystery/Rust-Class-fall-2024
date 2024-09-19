
//fn concat_strings(s1: &String, s2: &String) -> String {
    // Your code here
//}

//fn main() {
  //  let s1 = String::from("Hello, ");
 //   let s2 = String::from("World!");
   // let result = concat_strings(&s1, &s2);
    //println!("{}", result); // Should print: "Hello, World!"
//}

//fn main() {
  //let s1 = String::from("Hello");
  //let s2 = s1.clone();

  //println!("s1 is: {}, s2 is: {}", s1, s2);
//}
//---------------------------------------------------------------------------------------------------------
//fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
  
//let mut low: i32 = low; //first way to solve the problem
//while low <= high {
  //*total += low;
  //low += step;
//}

//for num: i32 in (low..high).step_by(step){ //second way to solve problem
  //*total += step;
//}

//}

//fn main() {
  //let mut result = 0;
  //sum_with_step(&mut result, 0, 100, 1);
  //println!("Sum 0 to 100, step 1: {}", result);

  //result = 0;
  //sum_with_step(&mut result, 0, 10, 2);
  //println!("Sum 0 to 10, step 2: {}", result);

  //result = 0;
  //sum_with_step(&mut result, 5, 15, 3);
  //println!("Sum 5 to 15, step 3: {}", result);
//}
//---------------------------------------------------------------------------------------

//fn most_frequent_word(text: &str) -> (String, usize) {
  //let words: Vec<&str> = text.split_whitespace().collect();
 //let word: &str = "quick";
 //let mut cnt: i32 = 0;

  //for idx: usize in 0..words.len(){
    //if words[idx]==word{
      //cnt +=1;
    //}
    
  //}
   //println!("{}",words[idx]);
  //  return ("the".to_string(),3);
  //(max_word, max_count) // return tuple
//}

//fn main() {
  //let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
  //let (word, count) = most_frequent_word(text);
  //println!("Most frequent word: \"{}\" ({} times)", word, count);
//}
//fn most_frequent_word(text: &str) -> (String, usize){
 // let words: Vec<&str> = text.split_whitespace().collect();
  //let mut max_word: &str = "";
//let mut max_count: usize
//}


/*use std::io;

fn main() {
    println!("Enter temperature in Fahrenheit: ");
    let mut input_f = String::new();
    io::stdin().read_line(&mut input_f).expect("Failed to read input");
    
    let far: f64 = input_f.trim().parse().expect("Please enter a valid number");
    let conv_cel = (far - 32.0) * 5.0 / 9.0;
    println!("Your temperature in Celsius is: {:.2}", conv_cel);
    println!("Enter temperature in Celsius: ");
    let mut input_c = String::new();
    io::stdin().read_line(&mut input_c).expect("Failed to read input");
    let cel: f64 = input_c.trim().parse().expect("Please enter a valid number");
    let conv_far = (cel * 9.0 / 5.0) + 32.0;
    println!("Your temperature in Fahrenheit is: {:.2}", conv_far);
}*/





/*fn main() {
  let arr = [3, 7, 9, 11, 15, 16, 20, 25, 32, 45];
  
  is_even(&arr);
  full_sum(&arr);
  big_num(&arr);
}

fn is_even(arr: &[i32]) {
  for &number in arr {
      if number % 3 == 0 && number % 5 == 0 {
          println!("FizzBuzz");
      } else if number % 2 == 0 {
          println!("{} is even", number);
      } else if number % 3 == 0 {
          println!("Fizz");
      } else if number % 5 == 0 {
          println!("Buzz");
      } else {
          println!("{} is odd", number);
      }
  }
}

fn full_sum(arr: &[i32]) {
  let total_sum: i32 = arr.iter().sum();
  println!("Your total sum is: {}", total_sum);
}

fn big_num(arr: &[i32]) {
  let mut big = arr[0];
  for &number in arr {
      if number > big {
          big = number;
      }
  }
  println!("Your biggest number is: {}", big);
}*/


use std::io;

fn main() {
    let secret = 32;
    check_guess(secret);
}

fn check_guess(secret: i32) {
    let mut count = 0;  

    loop {
        
        println!("Enter your guess: ");

        let mut guess = String::new();

        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue; 
            }
        };

        count += 1; 

        if guess == secret {
            println!("Your guess is correct!");
            break; 
        } else if guess > secret {
            println!("Your guess is too high.");
        } else {
            println!("Your guess is too low.");
        }
    }

    println!("Total guesses: {}", count); 
}
