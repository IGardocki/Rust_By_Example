use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First slice element: {}", slice[0]);
    println!("The slice has {} elements", slice.len())
}

fn main() {
    //fixed size arr. type signature is superfluous
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    //initializing all values to 0
    let ys: [i32; 500] = [0; 500];

    // arrs are zero indexed
    println!("first arr element: {}", xs[0]);
    println!("Second arr elememnt: {}", xs[1]);

    //len returns arr length
    println!("There are {} elements in the array", xs.len());

    // arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    //arrs can automatically be borrowed as slices
    println!("Borrow whole arr as slice.");
    analyze_slice(&xs);

    //slices can point to section of arr
    // syntax is [starting_index..ending_index]
    //ending_index is not inclusive
    println!("Borrow section of arr as slice.");
    analyze_slice(&ys[1 .. 4]);

    //example of empty slice: &[]
    let empty_arr: [u32; 0] = [];
    assert_eq!(&empty_arr, &[]);
    assert_eq!(&empty_arr, &[][..]); // same as above but more verbose

    //arrs can be safely accessed using .get, which returns an Option.
    //This can be matched like below or used with .expect if you want 
    //program to exit

    for i in 0..xs.len()+1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // out of bound indexing on arr causes compile time error
    // println!("{}", xs[5]);

    // Out of bound indexing on slice causes runtime error.
    //this is bc we dont how hob slice will be at compile time
    // println!("{}", xs[..][5]);
}

