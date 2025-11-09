fn main() {
    let mut var: &[i32] = &[0,1,4]; // 'var' continues to be a slice reference.
    let owned_boxed_slice: Box<[i32]>; // Declared in outer scope to ensure longevity.
    println!("{:?}", var);

    {
        let new_array = [0, 3, 4];
        // Convert the stack array to a heap-allocated Box<[i32]>.
        // Ownership of this Box is moved to 'owned_boxed_slice'.
        owned_boxed_slice = new_array.into_iter().collect::<Vec<i32>>().into_boxed_slice();
        var = &owned_boxed_slice;
    }
    // Now 'owned_boxed_slice' lives in the outer scope.
    // We can safely take a slice reference from it.
    // var = &owned_boxed_slice;

    println!("{:?}", var);
}