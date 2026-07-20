use rand::seq::IndexedRandom;
use rand::rng;

pub fn end_quote() {
    let quotes= [
        "Despite everything, it's still you.",
        "Autobots, roll out!",
        "Talk, talk, it's only talk.",
        "I'll be here in the dark...",
        "fedora-update Kept It Simple, Stupid!",
        "One... Two... Alyx... Wait-",
        "Danced out with the moonlit knight. Proceed!",
        "sudo? I hardly know her!",
        "Bash is easier, they said.",
        "If you found a bug, no you didn't.",
        "Hey you, you're finally... well, you just went to sleep again.",
        "Rust is easier, they... Wait-"
    ];
    if let Some(quote) = quotes.choose(&mut rand::rng()) {
        println!("=={}==", quote);
    }
}

// quotes are flavor text that go in the end of each update.
// feel free to add more, as long as it follows the guidelines below.
//
//                      guidelines to add quotes:
//
// 1. NO POLITICS
// 2. NO RACISM
// 3. NO HOMOPHOBIA, TRANSPHOBIA, MISOGYNY
// 4. NO XENOPHOBIA
// 5. NO DARK HUMOR
// 6. BE CAREFUL WITH SATIRE
//
// as much as I like dark humor and satire from time to time, it can easily be misinterpreted and
// genuinely offend someone. every new quote entry is going to be supervised personally.
//
// remember: this is a public project that people from different backgrounds and perspectives
// are going to use. write quotes wisely.