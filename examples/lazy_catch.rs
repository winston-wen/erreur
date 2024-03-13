use erreur::*;
use rand::{rngs::ThreadRng, Rng};

/// Demonstrate "lazy catch". The program only propagates the error message.
fn main() -> Resultat<()> {
    let mut rng = rand::thread_rng();

    let even = rand_even(&mut rng).catch_()?;
    println!("{}", even);

    let odd = rand_odd(&mut rng).catch_()?;
    println!("{}", odd);

    Ok(())
}

fn rand_even(rng: &mut ThreadRng) -> Resultat<u64> {
    let n: u64 = rng.gen_range(1..=1000_0000);
    assert_throw!(
        n % 2 == 0,                   // [required] boolean expression
        "UnluckyException",           // [optional] title
        format!("{} is not even", n)  // [optional] error message
    ); // Note that if exactly one optional arg is given, the arg is treated as **error message**.
    Ok(n)
}

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
