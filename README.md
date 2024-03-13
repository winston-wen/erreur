# Quick Start

After reading through this article, you will master the usage of:
* `Resultat<T>` as return type
* `assert_throw!(...)`
* `throw!(...)`
* `.catch()?` and `.catch_()?`
* `.ifnone()?` and `.ifnone_()?` 

## 1. Example of returning `Resultat<T>`

```rust 
use erreur::*;

pub fn rand_even() -> Resultat<u64> {
    // ... function body here ...
    Ok(1)
}
```

NOTE: If you are writing your own fail-able function, let it return `Resultat<T>`. 

## 2. Example of `assert_throw!(...)`

```rust
use erreur::*;

fn rand_even(rng: &mut ThreadRng) -> Resultat<u64> {
    let n: u64 = rng.gen_range(1..=1000_0000);
    assert_throw!(
        // [required] boolean expression
        n % 2 == 0,
        // [optional] title          
        "UnluckyException",     
        // [optional] error message      
        format!("{} is not even", n)  
    );
    Ok(n)
}
```

NOTE: if **exactly one** optional arg is given, the arg is treated as **error message**, and the title is automatically set to `"AssertionFailedException"`.

## 3. Example of `throw!(...)`

```rust
fn rand_odd(rng: &mut ThreadRng) -> Resultat<u64> {
    let n: u64 = rng.gen_range(1..=1000_0000);
    if n % 2 == 1 {
        return Ok(n);
    } else {
        throw!(
            // [required] title.
            "UnluckyException",
            // [required] error message.
            format!("{} is not odd", n)
        );

        // throw!(); // Lazy variant
        // throw!("DummyException", ""); // equivalent
    }
}
```

## 4. Example of `.catch(...)?` and `.catch_()?`

### 4.1. Full catch

This is extremely useful when the underlying error message is confusing. The programmer is responsible for customizing helpful error message.

```rust
use erreur::*;
use std::fs::File;

fn main() -> Resultat<()> {
    // `File::open` will show the following message on this path
    // -- "No such file or directory (os error 2)".
    // When you see this message in real business, 
    // --  you have no idea which file is missing.
    let path = "/impossible/path/!@#$%^&*()_+.file";

    // `catch` the `Result` returned by `open`,
    // write helpful message in `catch`.
    let _file = File::open(path).catch("CannotOpenFile", path)?;

    Ok(())
}
```

### 4.2. Lazy catch

If the underlying error message is helpful enough, use lazy catch to track the call stack and propagate the error message.

```rust
use erreur::*;
use rand::{rngs::ThreadRng, Rng};

fn main() -> Resultat<()> {
    let mut rng = rand::thread_rng();

    let even = rand_even(&mut rng).catch_()?;
    println!("{}", even);

    let odd = rand_odd(&mut rng).catch_()?;
    println!("{}", odd);

    Ok(())
}
```

## 5. Example of `.ifnone(...)?` and `.ifnone_()?`

If an `Option::None` stucks your business, call `.ifnone(...)?` or `.ifnone_()?` to throw an error.

```rust
use std::collections::HashMap;

use erreur::*;
use rand::Rng;

fn main() -> Resultat<()> {
    let mut rng = rand::thread_rng();

    let zoo = init_dict();
    let dice = rng.gen_range(1..=6);

    let animal = zoo
        .get(&dice)
        .ifnone(
            "UnluckyException",
            format!("dice = {}", dice),
        )?;
    // there's also a lazy variant: `.ifnone_()?`
    println!("{}", animal);

    Ok(())
}

fn init_dict() -> HashMap<i32, String> {
    let mut dict = HashMap::new();
    dict.insert(1, "bear".to_string());
    dict.insert(3, "kangaroo".to_string());
    dict.insert(5, "cockatoo".to_string());
    dict
}
```
