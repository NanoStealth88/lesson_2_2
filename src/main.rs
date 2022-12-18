fn main() {
    
     /* ---------------------------------
                VARIABLES IN RUST
     -----------------------------------*/

    //let x:i64 = 15;     // variable is immutable and can only be a letter or an underscore.  This appears to be like a const in c++

    let mut x:i64 = 15;     //this variable declaration allows this variable to be overwritten so this is like a normal variable in python or c++

    println!("The value of the variable x = {}", x);

    x = 22;   

    println!("The value of the variable x = {}", x);

    let y:i32 = 5*5;

    println!("y = {}", y);

    println!("\n");
    println!("SCALAR DATA TYPES");
    println!("");

    // 3 rules
    // variable name can consist of letters, numbers and underscore characters only. 
    // variable names must start with a letter or underscore.  never a digit
    // capitalization makes a different variable.   x and X are two different variables.

    /* ---------------------------------
            Data Types 
                Scalar Data Types
                    Integer
                        Signed  i8, i16, i32, i64
                            -2^(i-1)-1 to 2^(i-1)-1
                            -2^(8-1)-1 to 2^(8-1)-1 = -127 to 127
                        Unsigned */


    // use function MAX from the standard library which is included in all the programs by default
    // SIGNED integer data types and max size
    println!("SIGNED Integer maximum size");
    println!("The maximum number in i8 is equal to {}", std::i8::MAX);
    println!("The maximum number in i16 is equal to {}", std::i16::MAX);
    println!("The maximum number in i32 is equal to {}", std::i32::MAX);
    println!("The maximum number in i64 is equal to {}", std::i64::MAX);
    println!("");

    // UNSIGNED integer data types and max size
    println!("UNSIGNED Integer maximum size");
    println!("The maximum number in u8 is equal to {}", std::u8::MAX);
    println!("The maximum number in u16 is equal to {}", std::u16::MAX);
    println!("The maximum number in u32 is equal to {}", std::u32::MAX);
    println!("The maximum number in u64 is equal to {}", std::u64::MAX);

    /* --------------------------
            Floats
                f32
                f64
    ----------------------------*/

    let my_pi: f64 = 3.14159;

    //println!("The addition of an integer and a flost is {}", x+my_pi);  // cannot add integer and float together.

    println!("The maximum number in f32 is {}", std::f32::MAX);


}
