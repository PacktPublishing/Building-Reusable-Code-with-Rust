#![no_main]

struct HasSizedT<T>(T); // T: Sized by default
struct NotSizedT<T: ?Sized>(T);

// [i32] is not Sized
struct NoOK(HasSizedT<[i32]>); 
//          ^^^^^^^^^^^^^^^^^ `[i32]` does not have a constant size known at compile-time

struct OK(NotSizedT<[i32]>);
