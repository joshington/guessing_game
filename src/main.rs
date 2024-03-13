//====since we have added the rand crate lets use it
extern crate rand;//lets Rust know that we'll be using the rand crate
//==as an external dependency====
//guessing game, allow the player to input a guess
use std::io;  //to obatin user input and then print result as output, we need to bring the io
use std::cmp::Ordering; //***introducing a type called Ordering into scope from the std library,
//Orderign is another enum, but the variants for Ordering are Less, Greater, and Equal. 
use rand::Rng; //here we call anythiing in the rand crate by placing 
//rand:: before it
//Rng trait defines mthds that random nos. generators implement, and this 
// trait must be in scope for us to use those mthd.
//into scope.the io library comes from the standard library
fn main() {
    println!("Guess the number!");
	//adding this line here====
	let secret_number = rand::thread_rng().gen_range(1,101);
//rand::thread_rng function will give us the particular random number generator that we're going to use:one that is 
//local to the current thread of execution and seeded by the OS
//call the gen_range mthd, on the random no. generator. this mthd is defined by the Rng trait that we brought into 
//scope with the use rand::Rng statement. gen_range mthd takes 2 numbers as args and generates a random no. bthn
//them. its inclusive on the lowe bound but exclusive on the upper bound, thus 101, to select a no. btn 1 and 100
	//println!("The secret number is: {}", secret_number);
	loop {
		println!("Please input your guess.");
	
		let mut guess = String::new(); //create a place to store the userinput and expected to be astring.
		//String::new() - returns a new instance of a string, new function creates a new empty string
		//summary==> above creates a mutable variable that is currently bound to a new, empty instance of a String
		io::stdin().read_line(&mut guess)// this calls the read_line mthd on the standard input handle to get input from the user.
	//we're also passing one arg to read_line:&mut guess, job is to take whatever the user types into standard input and place
	//that into a string, & indicates that this arg is a reference which gives u a way to let multiple parts of the code
	//access one piece of data without needing to copy that data into memory nultiple times.
	//have to write &mut guess rather than &guess to make it mutable.
			.expect("Failed to read line");//if you dont call expect, program will compile, but you'll get a warning.
	//==ultimately we want to convert the String the program reads as input into a real number type so we can 
	//==compare it numerically to the guess.
		let guess: u32 = match  guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
			//if parse is able to successfully turn the string into a number, it will return an Ok value that contains
			//the resulting no. the Ok value will match the first arm's pattern, and the match expression will just return the num value
			//that parse produced and put inside the Ok value 
			// _, is a catchall value, we're saying we want to match all Err values, no matter what info they have inside them. so program
			//will execute the secondarm's code, continue, which tells the program to go 
		};
			//.expect("Please type a number!");
	//we've created a variable named gues. Rust allows us to shadow the revious value of guess with a new one.This feature is often 
	//used in situations in which u want to convert a value from one type to another type. shadowing lets us reuse the guess variable
	//name rather than forcing us to create 2 unique variables, such as guess_str and guess, 
	//===we bind guess to the expression guess.trim().parse().the guess in the expression refers to the original guess that was a
	//a string with the input in it.

	//==the trim mthd on a String instance will eliminate any whitespace at the beginning and end.although , trim eliminates \n 
	//which was added by pressing enter button.parse mthd on strings parses a string into some kind of number.because this mthd can 
	//parse a variety of no. types, we need to tell rust the exact no. type we want by using let guess:32
		println!("You guessed: {}", guess);
	//cmp mthd compares 2 values and can be called on anything that can be compared. it takes a reference
	//to whatever u intend to compare with. here its comparing the guess to the secret no.
	//==returns 
		match guess.cmp(&secret_number){
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	//a match expression is made up of arms.an arm consists of a pattern and the code that should be run
	//if the value given to the beginning of the match expression fits that arm's pattern.rust takes the value
	//given to match and looks thru each arm's pattern in turn.The match construct and patterns are powerful 
	//features in Rust that let u express a variety of situations your code might encounter and make sure that
	//you handle them all.
	}
}
//testing the build
