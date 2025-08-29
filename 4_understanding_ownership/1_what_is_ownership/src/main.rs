/*******************************************************************************************
*
*   "Safety" is the absence of undefined behavior. And a foundational goal of Rust is to
*   ensure that your programs never have undefined behavior, meaning, they are "safe".
*
*   A secondary goal of rust is to prevent undefined behavior at compile-time instead of
*   run-time.
*
*   Ownership is a discipline for safely using memory in Rust.
*
*   Variables live in frames. A frame is a mapping from variables to values within a single
*   scope, such as a function.
*
*   Frames are organized into a stack of currently-called-functions. After a function
*   returns, Rust deallocates the function's frame. Deallocating can be used interchangeably
*   with freeing and dropping.This sequence of frames is called a stack because last frame 
*   added ìs always the next frame freed.
*
*   To transfer access to data without copying it, Rust uses pointers. A pointer is a value
*   that describes a location in memory. The value that a pointer points-to is called its
*   pointee. One common way to make a pointer is to allocate memory in the heap.
*   The heap is a separate region of memory where data can live indefinitely. Heap data is
*   not tied to a specific stack frame. Rust provides a construct called Box for putting
*   data on the heap.
*
*   When a variable is bound to a box, we that the varible owns the box. Then if a owns a
*   box, let b = a moves ownership of the box from a to b.
*
*   Box deallocation priciple: if a variable owns a box, when Rust deallocates the variable's
*   frame, then Rust deallocates the box's heap memory.
*
*   Moved heap data principle: if ownershipt of heap data is moved from variable x to y,
*   then x cannot be used after the move.
*
********************************************************************************************/

fn main() {
    let n = 5; // L1
    let y = plus_one(n); // L3
    println!("The value of y is: {y}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // L2
}

// L1: Stack holds the frame of the main function wich holds n = 5.
// L2: Adds the frame of the plus_one fuction to the stack, wich holds x = 5.
// L3: Drops the frame of the plus_one function, and now the frame for the main
// function holds n = 5 and y = 6.