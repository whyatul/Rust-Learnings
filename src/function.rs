fn main(){ //main Function
    let sentence: String = String::from("Hello! my name is Atul");
    let first_func: String = get_first_func(sentence);
    println!("The Function Returns the value the first word of the sentence: {}", first_func);
}
fn get_first_func(sentence: String) -> String {
    let mut ans: String = String::from("");
    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());
        if char == ' ' {            
            break;
        }
    }
    return ans;
}