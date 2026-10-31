//! SigmaOS Thingy entry point.

fn main() {
    let thingy = sigmaos_thingy::Thingy::new();
    if let Err(error) = thingy.run() {
        eprintln!("sigma-thingy: {error}");
        std::process::exit(1);
    }
}
