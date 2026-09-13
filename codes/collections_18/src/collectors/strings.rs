fn new_string(){
    let mut s = String::new();
}

fn tostringtype(){

    let data = "initial contents";

    let s = data.to_string();

    //or
    let s = "initial contents".to_string();

    let s = String::from("inital contents.");

}

fn appending_at_str(){

    let mut s = String::from("foo");
    s.push_str("bar");
}

fn pushstrdoesnottakeownership(){

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
}

fn pushtakessinglechar(){

    let mut s = String::fom("lo");
    s.push('l');
}

fn combiningstrings(){

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

}

fn combiningproblematicstrings(){


    //way 1
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    //Way 2
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");


    //Way 1 takes ownership of s1 but way2 does not take anyones ownership. 
}

fn iteratingoversstrings(){
for c in "Зд".chars() {
    println!("{c}");

}

//or

for b in "Зд".bytes() {
    println!("{b}");
}

// But at other languages some of them stored with 2 bytes so going with char seems more ok to me. 
}


