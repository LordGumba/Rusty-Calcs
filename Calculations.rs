use std::io;

Z = 1
//Finds the factorial of the number by multiplying it by it minus one, until the number is 
fn factorial(X) {
    if n == 0{
        1;
    }
    else {
        answer = n * factorial(n-1);
    }
//Multiplies X with Y
fn multiply(X, Y){
    answer = X * Y
}
//Divides X with Y
fn divide(X, Y){
    answer = X / Y
}
// Adds X to Y
fn addition(X, Y){
    answer = X + Y
}
// Minuses Y from X
fn subtraction(X, Y){
    answer = X - Y
}
//Multiplies X by itself S number of times
fn square(X, S){

    if s == 0{
        answer = Z
        1;
    }
    else{
        s = s - 1
        Z = Z * X
        
    }
    
}
//Uses an If else statement to sort through the possible functions and find the correct one, then collects the numbers needed for input
fn FindFunction(){
    if function == "Factorial" {
        //Gets number to factor
        println!("Please enter your number:");
        let mut X = Int::new();
        io::stdin().read_line(&mut X)
            .expect("Failed to read line");
        //factors it
        factorial(X);
    }
    else if function == "Multiply" {
        //Get the first value
        println!("Please enter X:");
        let mut X = Int::new();
        io::stdin().read_line(&mut X)
            .expect("Failed to read line");
        //Get the second value
        println!("Please enter Y:");
        let mut Y = Int::new();
        io::stdin().read_line(&mut Y)
            .expect("Failed to read line");
        //Multiplies it
        multiply(X, Y);
    }
    else if function == "Divide" {
        //Get the first value
        println!("Please enter X:");
        let mut X = Int::new();
        io::stdin().read_line(&mut X)
            .expect("Failed to read line");
        //Get the second value
        println!("Please enter Y:");
        let mut Y = Int::new();
        io::stdin().read_line(&mut Y)
            .expect("Failed to read line");
        //divides them
        divide(X, Y);
    }
    else if function == "Addition" {
        //Get the first value
        println!("Please enter X:");
        let mut X = Int::new();
        io::stdin().read_line(&mut X)
            .expect("Failed to read line");
        //Get the second value
        println!("Please enter Y:");
        let mut Y = Int::new();
        io::stdin().read_line(&mut Y)
            .expect("Failed to read line");
        //adds them
        addition(X, Y);
    }
    else if function == "Subtraction" {
        //Get the first value
        println!("Please enter X:");
        let mut X = Int::new();
        io::stdin().read_line(&mut X)
            .expect("Failed to read line");
        //Get the second value
        println!("Please enter Y:");
        let mut Y = Int::new();
        io::stdin().read_line(&mut Y)
            .expect("Failed to read line");
        //subtracts them
        subtraction(X, Y);
    }
    else if function == "Square" {
        //Get the first value
        println!("Please enter the base number:");
        let mut X = Int::new();
        io::stdin().read_line(&mut X)
            .expect("Failed to read line");
        //Get the squaring value
        println!("Please enter the square:");
        let mut S = Int::new();
        io::stdin().read_line(&mut S)
            .expect("Failed to read line");
        //squares it
        square(X, S);
    }
        
}
}
//Asks guest to chose what mathamatical function they need, uses their inputted numbers to find it, and tells them the result.
fn main() {
    //Asks guest to chose what mathamatical function they need.
    println!("Please choose your function(Factorial, Multipy, Addition, Subtraction, Square):");
    let mut function = String::new();
    io::stdin().read_line(&mut function)
        .expect("Failed to read line");
    //Function finds the needed function, runs it, and finds the answer.
    FindFunction(function);
    
    //tells them the result.
    println!("The answer is", answer.trim());
}