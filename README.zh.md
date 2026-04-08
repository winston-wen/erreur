# 快速上手

阅读完本文后，你将掌握以下用法：
* `Resultat<T>` 作为返回类型
* `assert_throw!(...)`
* `throw!(...)`
* `.catch()?` 和 `.catch_()?`
* `.ifnone()?` 和 `.ifnone_()?`

## 1. 返回 `Resultat<T>` 示例

```rust
use erreur::*;

pub fn rand_even() -> Resultat<u64> {
    // ... 函数体 ...
    Ok(1)
}
```

注意：编写可能失败的函数时，请让其返回 `Resultat<T>`。

## 2. `assert_throw!(...)` 示例

```rust
use erreur::*;

fn rand_even(rng: &mut impl RngExt) -> Resultat<u64> {
    let n: u64 = rng.random_range(1..=1000_0000);
    assert_throw!(
        // [必填] 布尔表达式
        n % 2 == 0,
        // [可选] 异常名称
        "UnluckyException",
        // [可选] 错误信息
        format!("{} is not even", n)
    );
    Ok(n)
}
```

注意：如果只提供**一个**可选参数，该参数将被视为**错误信息**，异常名称会自动设为 `"AssertionFailedException"`。

## 3. `throw!(...)` 示例

```rust
fn rand_odd(rng: &mut impl RngExt) -> Resultat<u64> {
    let n: u64 = rng.random_range(1..=1000_0000);
    if n % 2 == 1 {
        return Ok(n);
    } else {
        throw!(
            // [必填] 异常名称
            "UnluckyException",
            // [必填] 错误信息
            format!("{} is not odd", n)
        );

        // throw!(); // 简写形式
        // throw!("UnknownException", ""); // 等价写法
    }
}
```

## 4. `.catch(...)?` 和 `.catch_()?` 示例

### 4.1. 完整 catch

当底层错误信息不够直观时非常有用。程序员负责自定义更有帮助的错误信息。

```rust
use erreur::*;
use std::fs::File;

fn main() -> Resultat<()> {
    // `File::open` 对这个路径只会显示
    // —— "No such file or directory (os error 2)"。
    // 在实际业务中看到这条消息时，
    // 你根本不知道是哪个文件缺失了。
    let path = "/impossible/path/!@#$%^&*()_+.file";

    // 用 `catch` 捕获 `open` 返回的 `Result`，
    // 在 `catch` 中填写有用的信息。
    let _file = File::open(path).catch("CannotOpenFile", path)?;

    Ok(())
}
```

### 4.2. 简写 catch

如果底层错误信息已经足够清晰，使用简写 catch 来跟踪调用栈并传播错误。

```rust
use erreur::*;
use rand::{Rng, RngExt};

fn main() -> Resultat<()> {
    let mut rng = rand::rng();

    let even = rand_even(&mut rng).catch_()?;
    println!("{}", even);

    let odd = rand_odd(&mut rng).catch_()?;
    println!("{}", odd);

    Ok(())
}
```

## 5. `.ifnone(...)?` 和 `.ifnone_()?` 示例

当 `Option::None` 阻断了你的业务逻辑时，调用 `.ifnone(...)?` 或 `.ifnone_()?` 抛出错误。

```rust
use std::collections::HashMap;

use erreur::*;
use rand::Rng;

fn main() -> Resultat<()> {
    let mut rng = rand::rng();

    let zoo = init_dict();
    let dice = rng.random_range(1..=6);

    let animal = zoo
        .get(&dice)
        .ifnone(
            "UnluckyException",
            format!("dice = {}", dice),
        )?;
    // 也有简写形式：`.ifnone_()?`
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
