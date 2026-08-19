fn main() {
    let x = 5;
    
    let x = x + 1;

    {
        let x = x * 2;
        println!("X at this scope is: {x}");
    } 

    println!("X at outer scope is: {x}");

    let spaces = "  ";
    let spaces = spaces.len();

    println!("length of spaces {spaces}");
    
}
