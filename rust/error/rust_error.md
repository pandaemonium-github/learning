
##

a simpler way to see results (Logan Smith)

## Mastering Error handling

[Mastering Error Handling in Rust: From Panics to thiserror & anyhow | with Nathan Stocks (https://www.youtube.com/watch?v=sZV6sz4P6QY&t=3605s)]  

Five rules:

### 1) enum

- Even if error might be defined as struct, best is to define it as an enum
- Name it xxxError

### 2) Group errors

There must be a good reason to create another error: an error can have many variants.
Create a new error for new domain.

### 3) Only your errors

Your library should return only your Error, which encapsulates all other third party errors.
It makes your interface more stable and does not expose your internal design.
You may return standard errors, like io::Error, if it makes sens.

### 4) Non exhaustive

```Rust
#[non_exhaustive]
enum SomeError {
  ...
}
```

This allows to add new variants without breaking user code.

### 5) Debug + Display + Error

```Rust
pub trait Error: Debug + Display
```

Error trait is defined to be a sub-trait of Debug and Display.

Implement Debug is easy, just derive enum Error from Debug.

For Display,

```Rust
use std::fmt::{Display, Formatter};

impl Display for MyError {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    use MyError::*;
    match self {
      InvalidInput(value) => write!(f, format!("{}: Input is invalid", value),
      ...
    }
  }
}
```

And then Error, which is straightforward:

```Rust
use std::error::Error;

impl Error for MyError {}
```

### 5b) Use thiserror

thiserror bring a derive macro Error, that implements the Display and Error trait for your Error.

```Rust
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
enum MyError {
  #[error("{0}: Input is invalid")]
  InvalidInput(value),
  #[error("Invalid name: {name}")]
  InvalidName{
    name: String
  }
  ...
}

```

## Use error with anyhow

```Rust
use anyhow::{Context, Result};

fn get_puzzle(filename: &str) -> Result<Puzzle> {
  let file = File::open(filename)
    .with_context(|| format!("couldn't open the puzzle file {}", filename))?;
  let puzzle = Puzzle::from_file(file).context("couldn't convert data into a puzzle")?;
  ...
}
```
