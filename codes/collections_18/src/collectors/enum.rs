enum SpreadsheetCell{
Int(i32),
Float(f64),
Text(String),
}

fn trials(){

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10,12),
    ];

    
}
