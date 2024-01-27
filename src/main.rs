use clap::Command;
fn main() {
    let _matches = Command::new("echor")
        .version("0.1.0")
        .author("Takumni Okamoto <takumi1988okamoto@gmail.com>")
        .about("Rust echo")
        .get_matches();
}
