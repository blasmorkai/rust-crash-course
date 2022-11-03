pub fn run() {
    println!("\nPointer_ref Section");
    
    // Primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;
     
    println!("Values array: {:?}", (arr1, arr2));

    // With Non-primitives, values are going to be stored in the heap
    //and this is where 'moved values' come into play

   // Primitive array
   let vec1 = vec![1,2,3];
   let vec2 = &vec1;  
   
   println!("Values vector: {:?}", (&vec1, vec2));
}
