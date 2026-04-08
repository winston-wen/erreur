# Démarrage rapide

Après avoir lu cet article, vous maîtriserez l'utilisation de :
* `Resultat<T>` comme type de retour
* `assert_throw!(...)`
* `throw!(...)`
* `.catch()?` et `.catch_()?`
* `.ifnone()?` et `.ifnone_()?`

## 1. Exemple de retour `Resultat<T>`

```rust
use erreur::*;

pub fn rand_even() -> Resultat<u64> {
    // ... corps de la fonction ...
    Ok(1)
}
```

NOTE : lorsque vous écrivez une fonction susceptible d'échouer, faites-la retourner `Resultat<T>`.

## 2. Exemple de `assert_throw!(...)`

```rust
use erreur::*;

fn rand_even(rng: &mut impl RngExt) -> Resultat<u64> {
    let n: u64 = rng.random_range(1..=1000_0000);
    assert_throw!(
        // [obligatoire] expression booléenne
        n % 2 == 0,
        // [facultatif] nom de l'exception
        "UnluckyException",
        // [facultatif] message d'erreur
        format!("{} is not even", n)
    );
    Ok(n)
}
```

NOTE : si **un seul** argument facultatif est fourni, il est traité comme **message d'erreur**, et le nom de l'exception est automatiquement défini à `"AssertionFailedException"`.

## 3. Exemple de `throw!(...)`

```rust
fn rand_odd(rng: &mut impl RngExt) -> Resultat<u64> {
    let n: u64 = rng.random_range(1..=1000_0000);
    if n % 2 == 1 {
        return Ok(n);
    } else {
        throw!(
            // [obligatoire] nom de l'exception
            "UnluckyException",
            // [obligatoire] message d'erreur
            format!("{} is not odd", n)
        );

        // throw!(); // Variante abrégée
        // throw!("UnknownException", ""); // équivalent
    }
}
```

## 4. Exemples de `.catch(...)?` et `.catch_()?`

### 4.1. Catch complet

Très utile lorsque le message d'erreur sous-jacent n'est pas clair. Le programmeur est responsable de fournir un message d'erreur personnalisé et utile.

```rust
use erreur::*;
use std::fs::File;

fn main() -> Resultat<()> {
    // `File::open` affichera le message suivant pour ce chemin :
    // — « No such file or directory (os error 2) ».
    // En production, ce message ne permet pas de savoir
    // quel fichier est manquant.
    let path = "/impossible/path/!@#$%^&*()_+.file";

    // Intercepter le `Result` retourné par `open`
    // et fournir un message utile via `catch`.
    let _file = File::open(path).catch("CannotOpenFile", path)?;

    Ok(())
}
```

### 4.2. Catch abrégé

Si le message d'erreur sous-jacent est suffisamment explicite, utilisez le catch abrégé pour suivre la pile d'appels et propager l'erreur.

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

## 5. Exemples de `.ifnone(...)?` et `.ifnone_()?`

Lorsqu'un `Option::None` bloque votre logique métier, appelez `.ifnone(...)?` ou `.ifnone_()?` pour lever une erreur.

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
    // il existe aussi une variante abrégée : `.ifnone_()?`
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
