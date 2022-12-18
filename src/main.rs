fn main() {
    
     /* ---------------------------------
                VARIABLES IN RUST
     -----------------------------------*/

    //let x:i64 = 15;     // variable is immutable and can only be a letter or an underscore.  This appears to be like a const in c++

    let mut x:i64 = 15;     //this variable declaration allows this variable to be overwritten so this is like a normal variable in python or c++

    println!("The value of the variable x = {}", x);

    x = 22;   

}
