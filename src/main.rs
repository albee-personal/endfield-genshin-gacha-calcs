use rand::rng;

use crate::{
    endfield::{EndfieldGachaState, endfield_pull_untilrateup},
    genshin::{GenshinGachaState, genshin_pull_untilrateup},
};

mod endfield;
mod genshin;

fn main() {
    let mut rng = rng();

    let budget = 1_000_000;

    // simulate endfield.
    println!("Endfield:");
    let mut endfield_state = EndfieldGachaState::default();
    let mut endfield_budget = budget;

    while endfield_budget > 0 {
        endfield_pull_untilrateup(&mut endfield_budget, &mut endfield_state, &mut rng);
        // println!("Got a rateup 6* at {} pulls!", endfield_state.total_pulls);
    }

    // simulate genshin
    println!("Genshin:");
    let mut genshin_state = GenshinGachaState::default();
    let mut genshin_budget = budget;

    while genshin_budget > 0 {
        genshin_pull_untilrateup(&mut genshin_budget, &mut genshin_state, &mut rng);
        // println!("Got a rateup 6* at {} pulls!", genshin_state.total_pulls);
    }

    dbg!(endfield_state);
    dbg!(genshin_state);
}
