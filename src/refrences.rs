fn main(){
    let s1 = String::from("Hello");
    let s2 = &s1; // & is the reference operator(an immutable borrow) rather than taking ownership or making a copy.

    println!("{}",s2);
    println!("{}",s1 );
}