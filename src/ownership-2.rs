fn main(){
    let mut s1 = String::from("hello");
    s1 = take_ownership(s1);
    println!("{}",s1 );
}

fn take_ownership(some_string: String) -> String{
    println!("{}",some_string );
    return some_string;
}