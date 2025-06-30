fn main() {
    let x: i32 = 4543;
    println!("The value of x is: {} This tells that x is a signed integer of 32 bits ", x);
    let y: f32 = 454.3000322;
    println!("The value of y is: {} This tells that x is a float of 32 bits ", y);

    
    //Booleans
    let is_male:bool = true;
    let is_18:bool = true;

    if(is_male){
        println!("The value of male");
    }
    else{
        println!("The vlaue is not male");
    }

    if is_18 && is_male{
        println!("The Presoon is legal to Vote");
    }

}
