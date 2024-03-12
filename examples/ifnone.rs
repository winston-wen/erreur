use std::collections::HashMap;

use erreur::*;
use rand::Rng;

fn main() -> Resultat<()> {
    let mut rng = rand::thread_rng();

    let zoo = init_dict();
    let dice = rng.gen_range(1..=6);

    let animal = zoo
        .get(&dice)
        .ifnone("UnluckyException", format!("dice = {}", dice))?;
    // there's also a lazy variant: `.ifnone_()`
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
