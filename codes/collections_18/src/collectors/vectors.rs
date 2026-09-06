pub fn health_check() {

 println!("trial");

}

pub fn creating_a_new_vector() {
    let v: Vec<i32> = Vec::new();

    v
}

pub fn creating_a_vector_with_macro() {
    let v = vec![1,2,3];

    v
}

pub fn updating_a_vector() {
    let mut v = Vec::new();

    v.push(3);
    v.push(4);
    v.push(5);
    v.push(6);

    v
}

pub fn reading_data_from_vector() {
    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    pritnln!("The third element is {third}");

    let third: Option<&i32> = v.get(2);

    match third {
        some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

pub fn iterating_over_vector() {
    let v = vec![100,32,57];

    for i in &v{
        println!("{i}");
    }

}