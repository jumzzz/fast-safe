# Error Handling

## On Aborting
From the book
```
Also, Rust’s panic behavior is customizable. If you compile with -C panic=abort , the
first panic in your program immediately aborts the process. (With this option, Rust
does not need to know how to unwind the stack, so this can reduce the size of your
compiled code.)
```

## On Catching Errors

```rust
match get_weather(hometown) {
    Ok(report) => {
        display_weather(hometown, &report);
    }
    Err(err) => {
        println!("error querying the weather: {}", err);
        schedule_weather_retry();
    }
}
```
This is Rust’s equivalent of try/catch in other languages. It’s what you use when you
want to handle errors head-on, not pass them on to your caller.

## On result Type Aliases

```rust
fn remove_file(path: &Path) -> Result<()>
```

## Using GenericError for Multiple Error Types
```rust
type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;
```

## Handling Errors in Main
You can change the type signature of main() to return a `Result` type.
```rust
fn main() -> Result<(), TideCalcError> {
    let tides = calculate_tides()?;
    print_tides(tides);
    Ok(())
}
```
If you have more complex error types, or want to include more details in your
message, it pays to print the error message yourself:

```rust
fn main() {
    if let Err(err) = calculate_tides() {
        print_error(&err);
        std::process::exit(1);
    }
}
```