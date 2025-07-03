fn main(){
    let s1 = String::from("hello");
    borrow_var(&s1); //passes a reference to s1
    println!("{}",s1 );//this is valid because the ownership was not transfered
}

fn borrow_var(some_string: &String){
    println!("{}",some_string );// some string is borrowed but not moved
   
}