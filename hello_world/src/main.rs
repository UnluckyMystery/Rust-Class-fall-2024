
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


/*use std::io;

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
}*/


//-------------------------------------------------------------------------------------------------------
//9/23/24

//missing a part of code here
/*#[derive(Debug)]
struct Car {
    color: String,
    make: String,
    year: u16,
}

impl Car {
    fn new(color: String, make: String, year: u16) -> Car {
        Car {
            color: color,
            make: make,
            year: year,
        }
    }

    fn honk_honk(&self) {
        // The &self is used to borrow the object, allowing this method to be called multiple times.
        println!("My car with color {} goes honk-honk!", self.color);
    }

    fn upgrade(&mut self, year: u16) {
        self.year = year;
        println!("Car upgraded to year {}", self.year);
    }
}

fn main() {
    let mut my_car = Car::new("black".to_string(), "BMW".to_string(), 2024);

    println!("{:?}", my_car); // Print the car details using debug formatting.
    my_car.honk_honk();        // Call the honk_honk method.
    
    my_car.upgrade(2025);      // Upgrade the car's year.
    println!("{:?}", my_car);  // Print the updated car details.
}*/

//---------------------------------------------------------------------------------------------------------------------------------
/*use std::mem;
struct Car {
    color: String,
    make: String,
    year: u16,
}

impl Car {
    fn new(color: String, make: String, year: u16) -> Car {
        Car {
            color: color,
            make: make,
            year: year,
        }
    }

    fn honk_honk(&self) {
        // The &self is used to borrow the object, allowing this method to be called multiple times.
        println!("My car with color {} goes honk-honk!", self.color);
    }

    fn upgrade(&mut self, year: u16) {
        self.year = year;
        println!("Car upgraded to year {}", self.year);
    }
}

fn main() {
  println!("Size of Car: {} bytes", mem::size_of::<Car>());
  println!("Alignment of Car: {} bytes", mem::align_of::<Car>());
}*/
//--------------------------------------------------------------------------------------------------------

/*use std::arch::asm;

fn main() {
    println!("Hello World");

    unsafe {
        // write syscall
        asm!(
            "mov rax, 1",  // syscall number for write
            "mov rdi, 1",  // file descriptor: 1 is stdout
            "syscall",
            in("rsi") message.as_ptr(),
            in("rdx") message.len(),
            out("rax") _,
            out("rcx") _,
            out("r11") _,
            clobber_abi("system")
        );

        // exit syscall
        asm!(
            "mov rax, 60", // syscall number for exit
            "xor rdi, rdi", // status code 0
            "syscall",
            options(noreturn)
        );
    }
}*/

//-----------------------------------------------------------------------------------------------------------------

/*use std::fs::File;
use std::io::Write;
fn create_and_write_to_file() {
    let mut file = File::create("example.txt").unwrap();
    println!("{:?}", file);
    writeln!(file, "Hello, Rust file operations!").unwrap();
    writeln!(file, "This is a new line.").unwrap();
}

fn main() {
    create_and_write_to_file();
    println!("File created and written successfully.");
}*/

//------------------------------------------------------------------------------------------------

/*use std::fs::File;
use std::io::{Read, BufReader, BufRead};

fn read_entire_file() {
    let mut file = File::open("example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("File contents:\n{}", contents);
}

fn read_file_line_by_line() {
    let file = File::open("example.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}

fn main() {
    println!("Reading entire file:");
    read_entire_file();

    println!("\nReading file line by line:");
    read_file_line_by_line();
}*/

//------------------------------------------------------------------------------------------------------------

//### Reading from Console

/*use std::fs::File;
use std::io::{self, Read, Write};

struct Person {
    name: String,
    year: u32,
}

fn reading_from_console() -> Person {
    let mut buffer = String::new();

    print!("What's your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();

    print!("What year is it from? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year: u32 = buffer.trim().parse().unwrap();
    buffer.clear();

    let car = Person { name, year };
    println!("A {}, that's from {} is a great car!", car.name, car.year);

    car
}

fn write_to_file(car: &Person) -> Result<(), io::Error> {
    let mut file = File::create("user_info.txt")?;
    writeln!(file, "Car Name: {}", car.name)?;
    writeln!(file, "Car Year: {}", car.year)?;
    writeln!(file, "Struct: Person {{ name: \"{}\", year: {} }}", car.name, car.year)?;
    Ok(())
}

fn read_entire_file() {
    let mut file = File::open("user_info.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("File contents:\n{}", contents);
}

fn main() {
    println!("Reading from console:");
    let car = reading_from_console();


    println!("Reading from file:");
    read_entire_file();
} */

/*use std::process::Command;

fn executing_os_commands_linux() {
    let output = Command::new("ls")
        .arg("-l")
        .output()
        .expect("Failed to execute command");

    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
}

fn main(){
    executing_os_commands_linux();
}*/

//------------------------------------------------------------------------------------------

/*#[derive(Debug)]
enum GradeLevel {
    Bachelor,
    Master,
    PhD,
}
#[derive(Debug)]
enum Major {
    ComputerScience,
    ElectricalEngineering,
}
#[derive(Debug)]
struct Student {
    name:String,
    grade:GradeLevel,
    major:Major
}

impl Student {
    fn new(name:String,grade:GradeLevel,major:Major) -> Self {
        Student {
            name:name,
            grade:grade,
            major:major,
        }
    }

    fn introduce_yourself(&self) {
        println!("Name: {}", self.name);

        match self.grade {
            GradeLevel::Bachelor => println!("Grade Level: Bachelor"),
            GradeLevel::Master => println!("Grade Level: Master"),
            GradeLevel::PhD => println!("Grade Level: PhD"),
        }

        match self.major {
            Major::ComputerScience => println!("Major: Computer Science"),
            Major::ElectricalEngineering => println!("Major: Electrical Engineering"),
        }
    }
}

fn main() {
    let s1 = Student::new(
        "John".to_string(),
        GradeLevel::Master,
        Major::ComputerScience);
    
    s1.introduce_yourself(); 
}*/


//------------------------------------------------------------------------------------------

/*use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
let mut file = File::create(filename).unwrap();

    for book in books {
        writeln!(file, "{},{},{}", book.title, book.author, book.year)
            .unwrap();
    }
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
}

fn load_books(filename: &str) -> Vec<Book> {
let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut books = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let fields: Vec<&str> = line.split(',').collect();

        if fields.len() == 3 {
            let title = fields[0].to_string();
            let author = fields[1].to_string();
            let year = fields[2].parse().unwrap();

            books.push(Book { title, author, year });
        }
    }

    books
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}*/
//--------------------------------------------------------------------------------------------------------


/*mod bank_account;

fn main() {
    let mut account = bank_account::BankAccount::new(100.0);
    println!("Initial balance: {}", account.balance());

    account.deposit(50.0);
    println!("Balance after deposit: {}", account.balance());

    account.withdraw(30.0);
    println!("Balance after withdrawal: {}", account.balance());
}*/
//-----------------------------------------------------------------------------------------------------------

/*use std::rc::Rc;
use std::cell::RefCell;

fn sharing_resource_refcell_count() {
    struct FamilyMember {
        tv: Rc<RefCell<TV>>,
    }

    #[derive(Debug)]
    struct TV {
        channel: String,
    }

    fn member_start_watch_tv() -> FamilyMember {
        let tv_is_on = Rc::new(RefCell::new(TV { channel: "BBC".to_string() }));
        FamilyMember { tv: tv_is_on }
    }

    let dad = member_start_watch_tv();
    let mom = FamilyMember { tv: Rc::clone(&dad.tv) };

    println!("TV channel for mom: {:?}", mom.tv.borrow());

    {
        let mut remote_control = dad.tv.borrow_mut();
        println!("TV channel before change: {:?}", remote_control);
        remote_control.channel = "MTV".to_string();
        println!("TV channel after change: {:?}", remote_control);
    } // The borrow_mut on remote_control ends here

    println!("TV channel for mom after dad's change: {:?}", mom.tv.borrow());
}

fn main() {
    sharing_resource_refcell_count();
}*/


//--------------------------------------------------------------------------------------------------------------------------

use ureq;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, Write};
use std::{thread, time};
use serde_json;

#[derive(Debug, Deserialize)]
struct Bitcoin {
    bpi: Bpi,
}

#[derive(Debug, Deserialize)]
struct Bpi {
    USD: Currency,
}

#[derive(Debug, Deserialize)]
struct Currency {
    rate_float: f64,
}

#[derive(Debug, Deserialize)]
struct Ethereum {
    bpi: Bpi,
}

#[derive(Debug, Deserialize)]
struct SP500 {
    chart: Chart,
}

#[derive(Debug, Deserialize)]
struct Chart {
    result: Vec<ResultItem>,
}

#[derive(Debug, Deserialize)]
struct ResultItem {
    meta: Meta,
}

#[derive(Debug, Deserialize)]
struct Meta {
    regularMarketPrice: f64,
}

trait Pricing {
    fn fetch_price(&self) -> Result<f64, String>;
    fn save_to_file(&self, price: f64) -> io::Result<()>;
}

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Result<f64, String> {
        let response = ureq::get("https://api.coindesk.com/v1/bpi/currentprice/BTC.json")
            .call()
            .map_err(|e| e.to_string())?;
        let data: Bitcoin = serde_json::from_str(&response.into_string().map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;  // Deserialize JSON from the response string
        Ok(data.bpi.USD.rate_float)
    }

    fn save_to_file(&self, price: f64) -> io::Result<()> {
        let mut file = File::create("bitcoin_price.txt")?;
        write!(file, "Bitcoin price: ${:.2}", price)
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&self) -> Result<f64, String> {
        let response = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd")
            .call()
            .map_err(|e| e.to_string())?;
        
        let data: serde_json::Value = serde_json::from_str(&response.into_string().map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;

        // Extract the price of Ethereum from the response
        let price = data["ethereum"]["usd"]
            .as_f64()
            .ok_or_else(|| "Failed to parse Ethereum price".to_string())?;

        Ok(price)
    }

    fn save_to_file(&self, price: f64) -> io::Result<()> {
        let mut file = File::create("ethereum_price.txt")?;
        write!(file, "Ethereum price: ${:.2}", price)
    }
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> Result<f64, String> {
        let response = ureq::get("https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d")
            .call()
            .map_err(|e| e.to_string())?;
        let data: SP500 = serde_json::from_str(&response.into_string().map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;  // Deserialize JSON from the response string
        Ok(data.chart.result[0].meta.regularMarketPrice)
    }

    fn save_to_file(&self, price: f64) -> io::Result<()> {
        let mut file = File::create("sp500_price.txt")?;
        write!(file, "S&P 500 price: ${:.2}", price)
    }
}
fn main() {
    let bitcoin = Bitcoin { bpi: Bpi { USD: Currency { rate_float: 0.0 } } };
    let ethereum = Ethereum { bpi: Bpi { USD: Currency { rate_float: 0.0 } } };
    let sp500 = SP500 {
        chart: Chart {
            result: vec![ResultItem {
                meta: Meta {
                    regularMarketPrice: 0.0,
                },
            }],
        },
    };

    loop {
        // Fetch and save Bitcoin price
        match bitcoin.fetch_price() {
            Ok(price) => {
                bitcoin.save_to_file(price).unwrap();
                println!("Bitcoin: ${:.2}", price);
            }
            Err(e) => println!("Error fetching Bitcoin price: {}", e),
        }

        // Fetch and save Ethereum price
        match ethereum.fetch_price() {
            Ok(price) => {
                ethereum.save_to_file(price).unwrap();
                println!("Ethereum: ${:.2}", price);
            }
            Err(e) => println!("Error fetching Ethereum price: {}", e),
        }

        // Fetch and save S&P 500 price
        match sp500.fetch_price() {
            Ok(price) => {
                sp500.save_to_file(price).unwrap();
                println!("S&P 500: ${:.2}", price);
            }
            Err(e) => println!("Error fetching S&P 500 price: {}", e),
        }

        // Sleep for 10 seconds
        thread::sleep(time::Duration::from_secs(10));
    }
}
