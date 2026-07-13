#[derive(Debug)]
enum Message {
    // 1- Variant without Data
    Empty,

    // 2- Tuple Variant
    Text(String),

    // 3- Struct Variant
    Number { value: i32, positive: bool },
}

fn main() {
    // -------------------------
    // Tuple Variant
    // -------------------------
    let msg1 = Message::Text(String::from("Ali"));

    match msg1 {
        Message::Text(text) => {
            println!("Text: {}", text);
        }

        Message::Number { value, positive } => {
            println!("Number: {} ({})", value, positive);
        }

        Message::Empty => {
            println!("Empty");
        }
    }

    // -------------------------
    // Struct Variant
    // -------------------------
    let msg2 = Message::Number {
        value: 100,
        positive: true,
    };

    match msg2 {
        Message::Text(text) => {
            println!("Text: {}", text);
        }

        Message::Number { value, positive } => {
            println!("Number: {}", value);
            println!("Positive: {}", positive);
        }

        Message::Empty => {
            println!("Empty");
        }
    }

    // -------------------------
    // Empty Variant
    // -------------------------
    let msg3 = Message::Empty;

    match msg3 {
        Message::Text(text) => {
            println!("Text: {}", text);
        }

        Message::Number { value, positive } => {
            println!("Number: {} ({})", value, positive);
        }

        Message::Empty => {
            println!("Empty");
        }
    }
}

//=========================================================================
//------------------------------------------------------------------------|
//=========================================================================
enum Signal {
    // Here we write the Data-type of Variant.
    Variant(String),

    Variant1,
    Variant2,
}

fn main() {
    // Create variable and but Value in it.
    let anything = Signal::Variant(String::from("Go"));

    match anything {
        // Here we write a new var which will take Variant.
        Signal::Variant(variable) => println!("{}", variable),

        Signal::Variant1 => println!("It's a 2nd"),

        Signal::Variant2 => println!("It's a 3rd"),
    }
}

//=========================================================================
//----------------------------| Result |----------------------------------|
//=========================================================================

fn main() {
    let login: Result<String, String> = Ok(String::from("Welcome Ali"));

    match login {
        Ok(message) => {
            println!("{}", message);
        }

        Err(error) => {
            println!("{}", error);
        }
    }
}

//=========================================================================
//------------------------------------------------------------------------|
//=========================================================================


// already have built in Rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Create Value
let value = Ok(data);
// or
let value = Err(error);

// Open the Value
match value {
    Ok(success_data) => {
        // use success data
    }

    Err(error_data) => {
        // use error data
    }
}
//==========================================+
// fn login() -> Result<User, String> { }   |
//------------------------------------------|
// Result< Option<User>, DatabaseError>     | 
//==========================================+

fn get_age() -> Result<i32, String> {
    Ok(20)
}

fn main() {
    // age-type: Result<i32, String>
    let age = get_age();

    // unwrap opens Result
    let value = age.unwrap();

    // value-type: i32
    println!("Age = {}", value);
}
//---------------------------------------------------

 fn some_function() -> Result<T, E> {
    Ok(...)
    // or
    Err(...)
}

fn main() {
    let result = some_function();

    let value = result.unwrap();

    println!("{:?}", value);
}

//=========================================================================
//------------------------------------------------------------------------|
//=========================================================================
              fn get_age()
                    │
                    ▼
        Result<i32, String>
                    │
                    ▼
                    ?
                 (match)
            ┌──────┴──────┐
            │             │
         Ok(20)      Err(error)
            │             │
            ▼             ▼
           20      return Err(error)
            │
            ▼
      let age: i32

//===========================================================================
// 
/* Stack
                           let x = Box::new(10);
┌─────────────────┐
│ x : Box         │
│ Pointer ────────┼────────────┐
└─────────────────┘            │
                               │
                               ▼
                              Heap
                     ┌────────────┐
                     │     10     │
                     └────────────┘
======================================================================
{
    let x = Box::new(10);
}
// End of scope.
// x (Box) dies.
// Box frees its data in Heap.

                Smart Pointers

         ┌─────────┼──────────┬──────────┐
         │         │          │          │
      String      Vec        Box        Rc / Arc
---------------------------------------------------------------
Smart Pointer

Pointer
│
├── Knows where data is.
│
└── Smart Pointer
    │
    ├── Knows where data is.
    └── Manages data life.      
*/
=============================================================
use std::boxed::Box;
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    box_example();

    rc_example();

    arc_example();
}

//==================================================

fn box_example() {

    // Create Box.
    let x = Box::new(10);

    // Data is in Heap.
    // Box is in Stack.

    println!("{}", x);

    // Dereference.
    // Get the value.
    println!("{}", *x);

}

// Box
// One owner.
// Data is in Heap.
// Box deletes data when it dies.

//==================================================

fn rc_example() {

    // Create Rc.
    let a = Rc::new(String::from("Ali"));

    // Do not copy data.
    // Add one more owner.
    let b = Rc::clone(&a);

    // New owner.
    // Not borrowing.

    println!("{}", a);
    println!("{}", b);

    // Owner count.
    println!("{}", Rc::strong_count(&a));

}

// Rc
// Reference Count.
// Many owners.
// One Thread.

//==================================================

fn arc_example() {

    // Create Arc.
    let a = Arc::new(String::from("Ali"));

    // Do not copy data.
    // Add one more owner.
    let b = Arc::clone(&a);

    // New owner.
    // Not borrowing.

    println!("{}", a);
    println!("{}", b);

    // Owner count.
    println!("{}", Arc::strong_count(&a));

}

// Arc
// Atomic Reference Count.
// Many owners.
// Many Threads.

//==================================================

// Pointer
// Hold an address.

// Box
// One owner.

// Rc
// Many owners.
// One Thread.

// Arc
// Many owners.
// Many Threads.
//===================| push('') and push_str("") |==============================
fn main() {
    // We make a mutable string. We can change it.
    let mut my_text = String::from("Roo");

    // "push_str()" adds a full text (a string slice).
    // Use double quotes "" for text.
    my_text.push_str("st"); 
    // Now my_text is "Roost"

    // "push()" adds only ONE letter (a character).
    // Use single quotes '' for a character.
    my_text.push('!'); 
    // Now my_text is "Roost!"

    println!("{}", my_text);
}
//=========================================================
//----------------------| Impl |---------------------------|
//==========================================================

struct Book {
    price: f32,
    title: String,
}
impl Book {
    fn print_title(&self) {
        println!("Title: {}", self.title);
    }
    fn print_price(&self) {
        println!("Price: {}", self.price);
    }
}

fn main() {
    let first_book = Books {
        price: 9.99,
        title: String::from("History"),
    };

    first_book.print_title();
    first_book.print_price();
}
