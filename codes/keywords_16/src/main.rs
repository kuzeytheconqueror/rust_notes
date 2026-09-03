use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result{
    
}

fn function2() -> IoResult<()> {

}

fn main() {

    let mut map = HashMap::new();
    map.insert(1, 2);

    println!("Hello, world!");
}
