use std::fmt;

// tuples can be used as fn args and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32){

    // let binds members of tuple to variable
    // this is similar to destructuring
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

fn transpose(mut matrix: Matrix) -> Matrix{
    let holder = matrix.1;
    matrix.1 = matrix.2;
    matrix.2 = holder;
    matrix
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn main() {

    //tuples can have tons of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, 
        -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

    // use tuple indexing to extract values
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuples second value: {}", long_tuple.1);

    //tuples can be in tuples
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8, -2i16));
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // will have compiler error if try to print tuple more than 12 elements
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long and throws error: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("pair is: {:?}", pair);

    let reversed_pair = reverse(pair);
    println!("reversed pair is: {:?}", reversed_pair);

    // to create one element tuple, comma is needed to tell them apart
    // from literal surrounded by parens
    println!("one element tuple: {:?}", (5u32,));
    println!("Just an int: {:?}", (5u32));
    
    //destructuring a tuple:
    let new_tuple = (1, "hello", 4.5, true);
    let (num, greeting, num_2, yes) = new_tuple;
    println!("{:?}, {:?}, {:?}, {:?}", num, greeting, num_2, yes);

    let matrix = Matrix(1.0, 2.0, 3.0, 4.0);
    println!("matrix:\n{}", matrix);
    println!("transposed matrix:\n{}", transpose(matrix))
}