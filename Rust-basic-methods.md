# Rust Basic Methods 

A small reference for the most common methods.

------------------------------------------------------------------------

## `Vec<T>` Description:

-  `vec.push(value)`     Add one item to the end.
- `vec.pop()`           Remove the last item.
- `vec.remove(index)`   Remove one item by index.
- `vec.len()`           Number of items.
- `vec.first()`         First item.
- `vec.last()`          Last item.
- `vec.iter()`          Read all items with Borrow,Doesn't take Ownership.

### Example

``` rust
let mut vec = vec![1, 2, 3];

vec.push(4);
vec.pop();
vec.remove(0);

println!("{}", vec.len());
println!("{:?}", vec.first());
println!("{:?}", vec.last());

for item in vec.iter() {
    println!("{}", item);
}
```

------------------------------------------------------------------------

## `String` Description:

- `text.push('A')`         Add one char.
- `text.push_str("Ali")`   Add one string.
- `text.trim()`            Remove spaces at start and end.
- `text.parse::<i32>()`    Change text to another type.
- `text.to_lowercase()`    Change text to small letters.
- `text.to_uppercase()`    Change text to big letters.
- `text.to_string()`       Change a value to String type.
- `text.lines()`           Split text into lines, each line one item.
- `text.contains("part")`  Check if text has a part inside, gives (true/false).

### Example

``` rust
let mut text = String::from("Ali");

text.push('!');
text.push_str(" Rust");

let clean = text.trim();

let number = "20".parse::<i32>().unwrap();
```
``` rust
let sentence = String::from("I love Rust");
let found = sentence.contains("Rust"); // true or false .
let text = String::from("Ali\nOmar\nSara");

let names = text.lines();

// Input: "Ali\nOmar\nSara"

// Output: Ali, Omar, Sara (each on its own line)
```

------------------------------------------------------------------------

## `Clone` Description :

  `value.clone()`   Make a new copy.

### Example

``` rust
let a = String::from("Ali");
let b = a.clone();
```

------------------------------------------------------------------------

## `iter`, `iter_mut` and `without iter`
##### `iter`
```rust
// make Borrowing to read items.
for item in vec_name.iter() {
    println!("{:?}", item);
}
```

##### `without iter`
```rust
// take Ownership from Vec.
for item in vec_name {
    println!("{:?}", item);
}
```

##### `iter_mut`
```rust
// reads item with &mut
for item in vec_name.iter_mut() {
    *item += 1; // change values
}
```



------------------------------------------------------------------------

## Quick Notes

-   `push()` → Add one item.
-   `push_str()` → Add one string.
-   `pop()` → Remove last item.
-   `remove()` → Remove by index.
-   `len()` → Number of items.
-   `first()` → First item.
-   `last()` → Last item.
-   `iter()` → Read all items without taking Ownership.
-   `trim()` → Remove spaces.
-   `parse()` → Change type.
-   `clone()` → Make a new copy.
-   `text.to_string()` → Change a value to String type.
-   `text.lines()` → Split text into lines, each line one item.
-   `text.to_lowercase()` → Change text to small letters.
-   `text.to_uppercase()` → Change text to big letters.
-   `text.contains("part")` → Check if text has a part inside, gives (true/false).
