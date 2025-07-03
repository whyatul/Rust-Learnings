struct User{
    active:bool,
    user_name: String,
    email: String,
    sign_in_count: u64,
}

fn main(){
    let user1 = User{
        active: true,
        user_name:String::from("Atul Maurya"),
        email: String::from("atulmaurya18.04@gmail.com"),
        sign_in_count: 43,
    };
    print!("User 1 Username {:?}",user1.user_name);
}