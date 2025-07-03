fn main(){
    let mut s1 = String::from("Hello There ");
    update_wor(&mut s1);
    println!("{}",s1);
}

fn update_wor(word: &mut String){
    word.push_str("world");
}