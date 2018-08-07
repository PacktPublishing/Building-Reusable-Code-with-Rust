use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex, RwLock};

fn main() {
    // Heap allocated pointers
    let mybox: Box<i32> = Box::new(42); // heap allocated memory
    let myrc: Rc<i32> = Rc::new(42); // reference-counted

    // Cell with internal mutability
    let mycell: Cell<i32> = Cell::new(42); 
    // mycell.set(0);
    // mycell.get();
    let myrefcell: RefCell<i32> = RefCell::new(42); // with read-write lock
    // myrefcell.borrow()
    // myrefcell.borrow_mut()

    // Thread-safe version
    let myarc: Arc<i32> = Arc::new(42); // automic reference count, thread-safe Rc<T>
    let mymutex: Mutex<i32> = Mutex::new(42); // automic reference count, thread-safe Rc<T>
    // mutex.lock()
    let myrwlock: RwLock<i32> = RwLock::new(42); // automic reference count, thread-safe Rc<T>
    // myrwlock.lock()

    // composition examples:
    // Rc<<RefCell<T>> 
    // Rc<Vec<RefCell<T>>> 
    
    // Reference: https://doc.rust-lang.org/book/first-edition/choosing-your-guarantees.html
}

