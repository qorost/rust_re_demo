use std::ptr;

/*
 *Data Races
 *Dereferencing a null/dangling raw pointer
 *Reads of undef (uninitialized) memory
 *breaking the pointer aliasing rules with raw pointers
 *&mut T and &T follow LLVM's scoped noalis model, except if the &T contains an UnsafeCell<U>. Unsafe code must not violate these aliasing guarantees.
 *Mutating an immutable value/reference without UnsafeCell<U>
 *Invoking undefined behavior via compiler intrinsics:
 *(1) Indexing outside of the bounds of an object with std::ptr::offset(offset intrinsic), with the exception of one byte past the end which is permitted.
 *(2) Using std::ptr::copy_nonoverlapping_memory(memcpy32/memcpy64 intrinsics) on overlapping buffers 
 *Invalid values in primitive types, even in private fields/locals:
 (1) NULL/dangling reference or boxes
 *(2) a value other than false(0) or true(1) in a bool
 (3) a discriminant in an enum not included in its type definition
 (4) A value in a char with a surrogate or about char::MAX
 (5) Non-UTF-8 byte sequences in a str
 * Unwiding into Rust from foreign code or unwinding from Rust into foreign code
 * * */


//Data Races

/*
 * In both unsafe functions and unsafe blocks , Rust will let you do three things that you normally
 * can not do, just three:
 * 1. Access or update a static mutable variable.
 * 2. Dereference a raw pointer
 * 3. Call unsafe functions. This is the most powerful ability
 *
 * */

// Access or update static mut
// Rust has a feature called 'static mut' which allows for mutable global state. Doing so can cause
// a data race and as such is inherently not safe.

static mut N: i32 = 5;
fn access_update_static_mut() {
    
    println!("static mut is {} ",N);

}


//Dereferencing a null/dangling raw pointer

fn derefer_null_raw_pointer() {
    let raw: *const i32 = ptr::null();
    assert!(raw.is_null());
    unsafe{
    
    println!("raw is {}",*raw);
    }

}


fn main() {
    derefer_null_raw_pointer();

}
