# Rust Basic Methods Cheat Sheet

A small reference for the most common methods.

------------------------------------------------------------------------

## `Vec<T>`

  Method                Description
  --------------------- ---------------------------
  `vec.push(value)`     Add one item to the end.
  `vec.pop()`           Remove the last item.
  `vec.remove(index)`   Remove one item by index.
  `vec.len()`           Number of items.
  `vec.first()`         First item.
  `vec.last()`          Last item.
  `vec.iter()`          Read all items.

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

## `String`

  Method                   Description
  ------------------------ ---------------------------------
  `text.push('A')`         Add one char.
  `text.push_str("Ali")`   Add one string.
  `text.trim()`            Remove spaces at start and end.
  `text.parse::<i32>()`    Change text to another type.

### Example

``` rust
let mut text = String::from("Ali");

text.push('!');
text.push_str(" Rust");

let clean = text.trim();

let number = "20".parse::<i32>().unwrap();
```

------------------------------------------------------------------------

## `Clone`

  Method            Description
  ----------------- ------------------
  `value.clone()`   Make a new copy.

### Example

``` rust
let a = String::from("Ali");
let b = a.clone();
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
-   `iter()` → Read all items.
-   `trim()` → Remove spaces.
-   `parse()` → Change type.
-   `clone()` → Make a new copy.
