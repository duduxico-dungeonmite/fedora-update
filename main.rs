use crate::quotes::end_quote;
use crate::update_dnf::update_dnf;

mod update_dnf;
mod quotes;

fn main() {
    let project = "fedora-update";
    let version = "2.0.0 | Development Branch";
    println!("Hello, from {} version {}!", project, version);
    update_dnf();
    end_quote();
}