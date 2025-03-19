use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Cell;
use std::boxed::Box;


pub fn lifeTime_box_pointer_example() {
    let str1 = String::from("hello");
    //now by using the reference of text we can use it in the struct
    // by giving life time, we are telling, MyString instance cannot out live text
    let my_string = MyString{text: str1.as_str()};
    println!("my_string: {}", my_string.text);

    let s: &'static mut str = "I have a static lifetime.";
    //& - common pointer
    
    //=======================================
        //Box - Smart pointer, allocates data on heap rather than stack
        //however the pointer to heap data is on stack
        //Box is used when we want to allocate memory on heap
        //Box is used when we want to return a value from a function
        //Box is used when we want to store a value in a data structure
    //================================================================
let t = (12, "eggs"); // a tuple created on the stack
let b = Box::new(t); // create on the heap, but b was stored on the stack
println!("b: {:?}", b);

let x = 5;
let y = &x;
let z = Box::new(x);
println!("x: {}, y: {}, z: {}", x, y, z);
assert_eq!(5, x);
assert_eq!(5, y);


let s1 = Rc::new("shirataki".to_string());
let s2 = s1.clone(); // doesnt copy value, instead increments the reference count
let s3 = s2.clone();
println!("s1: {}, s2: {}, s3: {}", s1.contains("Shira"), s2, s3); // s1, s2, s3 are all in stack, 
//pointing to the same memory location
// whereas the data is in heap memory

//REFCELL
let x = RefCell::new(42);

let flag = Flagger {is_true: RefCell::new(true)};
//borrow returns ref<T>
//borrow_mut returns refmut<T>

let reference = flag.is_true.borrow();
println!("reference: {}", reference);

    //Box is a smart pointer because it implements the Deref trait, which allows Box to be treated like a reference
    //Box is a smart pointer because it implements the Drop trait, which allows Box to clean up the heap memory when a Box goes out of scope
    //Box is a smart pointer because it implements the Debug trait, which allows Box to be printed with the {:?} format specifier
    //Box is a smart pointer because it implements the Clone trait, which allows Box to be cloned

    //Rc - Reference counting - allows multiple ownership


    //Ref - Interior mutability
    //RefMut - Interior mutability - accesssed through RefCell, which enforces 
    //the borrowing rules at runtime instead of compile time
    //Cell - Interior mutability - allows mutation of a value through a shared reference
    
    //Arc - Atomic reference counting
    //Cell - Interior mutability
    //RefCell - Interior mutability
    //Mutex - Thread safety
    //RwLock - Thread safety
    //Cow - Copy on write
    //Pin - Unmovable
    //PhantomData - Zero sized type
    //ManuallyDrop - Leak memory
    //NonZero - Non zero sized type
    //NonNull - Non null pointer
    //Unique - Unique pointer
    //Shared - Shared pointer
    //Weak - Weak pointer
    //Deref - Deref trait
    //Drop - Drop trait
    //Fn - Function pointer
    //FnMut - Function pointer
    //FnOnce - Function pointer
    //Future - Async programming
    //Stream - Async programming
    //Task - Async programming
    //JoinHandle - Async programming
    //AsyncRead - Async IO
    //AsyncWrite - Async IO
    //AsyncSeek - Async IO
    //AsyncBufRead - Async IO
    //AsyncWriteExt - Async IO
    //AsyncReadExt - Async IO
    //AsyncSeekExt - Async IO
    //AsyncBufReadExt - Async IO
    //AsyncWrite - Async IO
    //AsyncRead - Async IO
    //AsyncSeek - Async IO
    //AsyncBufRead - Async IO
    //AsyncWriteExt - Async IO
    //AsyncReadExt - Async IO
    //AsyncSeekExt - Async IO

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifeTime_example() {
        lifeTime_box_pointer_example();

    }
}