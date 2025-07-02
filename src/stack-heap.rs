fn main(){
    stack_fn(); //calls the function that usages stack memory
    heap_fn(); //call the function that usages heap memory
    updated_string(); //call the function that changes the size of the variable at runtime 
}

fn stack_fn(){
    let a = 10;
    let b = 24;
    let c = 54;
    let d = 90;
    let e = a + b + c + d;
    println!("Stack Memory Calling Function it is: {}",e);
}

fn heap_fn(){
    let s1 = String::from("Hello World!");
    let s2 = String::from("This is a Heap Memory Calling Function");
    let combined = format!("{} {}",s1,s2);
    println!("{}",combined );
}

//Starts with the base string on the heap
fn updated_string(){
    let mut s = String::from("Initial String");
    println!("Before Update: {}",s );
    println!("Capacity: {}, Length: {}, Pointer:{:p}",s.capacity(),s.len(),s.as_ptr() );

    //Appended Some text to the string
    s.push_str("And Some Additional Text");
    println!("After Appending {}",s);
    println!("Capacity: {}, Length: {}, Pointer:{:p}",s.capacity(),s.len(),s.as_ptr() );
}