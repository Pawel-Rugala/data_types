fn main() {
    // SCALAR TYPES
    // Integer
    // 2n-1 => 8bit => 2^8-1 => 255, 16bit => 2^16-1 => 65535...
    let x:i8 = 127; // 8-bit signed integer
    let x:i16 = 32767; // 16-bit signed integer
    let x:i32 = 2147483647; // 32-bit signed integer
    let x:i64 = 9223372036854775807; // 64-bit signed integer
    let x:i128 = 170141183460469231731687303715884105727; // 128-bit signed integer
    let x:isize = 9223372036854775807; // pointer size signed integer

    //number literals
    let oneMillion = 1_000_000; // decimal

    //Floating point
    let x:f32 = 3.14; // 32-bit floating point, single precision float
    let x:f64 = 3.14; // 64-bit floating point, double precision float

    //numeric operations
    let sum = 5 + 10; // addition
    let difference = 95.5 - 4.3; // subtraction
    let product = 4 * 30; // multiplication
    let quotient = 56.7 / 32.2; // division
    let truncated = -5 / 3; // truncated division result is -1 not -1.6666
    let remainder = 43 % 5; // remainder

    //Boolean
    let t = true;
    let f:bool = false; // with explicit type annotation
    let t = !true; // not => false

    //Character
    let c = 'z';
    let heart_eyed_cat = '😻';

    //Compound Types
    //Tuple -- have a fixed length: once declared, they cannot grow or shrink in size.
    let tup:(i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring, x = 500, y = 6.4, z = 1
    let five_hundred = tup.0; // accessing tuple element, 500

    let EmptyTuple = (); // empty tuple
    //The tuple without any values has a special name, unit. 
    //This value and its corresponding type are both written () 
    //and represent an empty value or an empty return type. 
    //Expressions implicitly return the unit value if they don’t return any other value.

    //Array -- have a fixed length: once declared, they cannot grow or shrink in size.
    let a:[i32; 5] = [1, 2, 3, 4, 5]; // array of 5 elements of type i32
    let a = [3; 5]; // array of 5 elements of type i32, all elements are 3
    let first = a[0]; // accessing array element, 3
}
