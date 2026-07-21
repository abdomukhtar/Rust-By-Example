# Rust `std::fs` and `std::env` Basic Functions

A small reference for the most common functions.

------------------------------------------------------------------------

## `std::fs` Description:

- `fs::read(path)`              Read file as bytes.
- `fs::read_to_string(path)`    Read file as String.
- `fs::write(path, data)`       Create or overwrite a file.
- `fs::copy(from, to)`          Copy one file.
- `fs::rename(old, new)`        Rename or move a file.
- `fs::remove_file(path)`       Delete one file.
- `fs::create_dir(path)`        Create one directory.
- `fs::create_dir_all(path)`    Create nested directories.
- `fs::remove_dir(path)`        Remove an empty directory.
- `fs::remove_dir_all(path)`    Remove directory and everything inside.
- `fs::read_dir(path)`          Read directory contents.
- `fs::metadata(path)`          Get file information.

### Example

```rust
use std::fs;

fs::write("note.txt", "Hello").unwrap();

let text = fs::read_to_string("note.txt").unwrap();

fs::remove_file("note.txt").unwrap();
```

------------------------------------------------------------------------

# Rust `std::env` Basic Functions
## `std::env` read the input at the same time that programme start
##### `std::env` good for CLI

A small reference for the most common functions.

------------------------------------------------------------------------

## `std::env` Description:

- `env::args()`                 Read command-line arguments.
- `env::args_os()`              Read arguments as OsString.
- `env::var(name)`              Read environment variable.
- `env::set_var(name, value)`   Create or change environment variable.
- `env::remove_var(name)`       Remove environment variable.
- `env::current_dir()`          Get current directory.
- `env::set_current_dir(path)`  Change current directory.
- `env::current_exe()`          Get executable path.
- `env::temp_dir()`             Get temporary directory.

### Example

```rust
use std::env;

println!("{:?}", env::current_dir());

let home = env::var("HOME").unwrap();
let var: Args = env::args();
```
```rust
let args: Vec<String> = env::args().collect();
```
```rust
for arg in env::args() {
    println!("{}", arg);
}
```
------------------------------------------------------------------------

# Rust `std::path` Basic Types & Methods

A small reference for the most common methods.

------------------------------------------------------------------------

## `Path` Description:

- `Path::new(path)`             Create a Path.
- `path.exists()`               Check if path exists.
- `path.is_file()`              Check if it is a file.
- `path.is_dir()`               Check if it is a directory.
- `path.file_name()`            Get file name.
- `path.extension()`            Get file extension.
- `path.parent()`               Get parent directory.
- `path.join(name)`             Join another path.
- `path.display()`              Print path nicely.

### Example

```rust
use std::path::Path;

let path = Path::new("note.txt");

println!("{}", path.exists());
```

------------------------------------------------------------------------

# Rust `std::process` Basic Functions

A small reference for the most common functions.

------------------------------------------------------------------------

## `std::process` Description:

- `Command::new(program)`       Start another program.
- `.arg(value)`                 Add one argument.
- `.args(values)`               Add multiple arguments.
- `.output()`                   Run and capture output.
- `.status()`                   Run and get exit status.
- `.spawn()`                    Run program in background.
- `process::exit(code)`         Exit current program.

### Example

```rust
use std::process::Command;

Command::new("ls")
    .arg("-l")
    .status()
    .unwrap();
```

------------------------------------------------------------------------

## Quick Notes

- `fs::read_to_string()` → Read file as String.
- `fs::write()` → Write file.
- `fs::copy()` → Copy file.
- `fs::rename()` → Rename or move file.
- `fs::remove_file()` → Delete file.
- `fs::read_dir()` → Read directory contents.

- `env::args()` → Read command-line arguments.
- `env::var()` → Read environment variable.
- `env::current_dir()` → Get current directory.
- `env::set_current_dir()` → Change current directory.

- `Path::new()` → Create a Path.
- `exists()` → Check if path exists.
- `is_file()` → Check if file.
- `is_dir()` → Check if directory.
- `join()` → Join paths.
- `extension()` → Get file extension.

- `Command::new()` → Start another program.
- `.arg()` → Add one argument.
- `.args()` → Add multiple arguments.
- `.output()` → Capture output.
- `.status()` → Get exit status.
- `.spawn()` → Run in background.
- `process::exit()` → Exit program.
