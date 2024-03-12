use erreur::*;

/// Demonstrates eager catch.
/// The prgrammer is responsible for customizing helpful error message.
fn main() -> Resultat<()> {
    let path = "/impossible/path/!@#$%^&*()_+.file";

    // The bloody `File::open` will show "No such file or directory (os error 2)".
    // When you see this message, you have no idea which file is missing.
    // Use eager catch to generate helpful error message .
    let _file = std::fs::File::open(path).catch("CannotOpenFile", path)?;

    Ok(())
}
