use rand::rng;

use crate::{
    endfield::{EndfieldGachaState, endfield_pull_untilrateup, endfield_pull_untilrateup_max_pot},
    genshin::{GenshinGachaState, genshin_pull_untilrateup, genshin_pull_untilrateup_max_pot},
};

mod endfield;
mod genshin;

fn main() {
    simulate_pulls(1_000_000);
    test_average_per_banner(10_000);
    test_average_per_banner_maxpot(10_000);
}

fn simulate_pulls(budget: i32) {
    let mut rng = rng();

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

fn test_average_per_banner(simulations: i32) {
    let mut rng = rng();

    let mut total_spent_endfield = 0;
    for _ in 0..simulations {
        let mut endfield_state = EndfieldGachaState::default();
        let mut endfield_budget = 1_000;

        endfield_pull_untilrateup(&mut endfield_budget, &mut endfield_state, &mut rng);
        let spent = endfield_state.total_pulls;

        total_spent_endfield += spent;
    }

    let mut total_spent_genshin = 0;
    for _ in 0..simulations {
        let mut genshin_state = GenshinGachaState::default();
        let mut genshin_budget = 1_000;

        genshin_pull_untilrateup(&mut genshin_budget, &mut genshin_state, &mut rng);
        let spent = genshin_state.total_pulls;

        total_spent_genshin += spent;
    }

    println!(
        "[ENDFIELD] Average per 1 rateup: {}",
        total_spent_endfield / simulations
    );

    println!(
        "[GENSHIN] Average per 1 rateup: {}",
        total_spent_genshin / simulations
    );
}

fn test_average_per_banner_maxpot(simulations: i32) {
    let mut rng = rng();

    let mut total_spent_endfield = 0;
    for _ in 0..simulations {
        let mut endfield_state = EndfieldGachaState::default();
        let mut endfield_budget = 100_000;

        endfield_pull_untilrateup_max_pot(&mut endfield_budget, &mut endfield_state, &mut rng);
        let spent = endfield_state.total_pulls;

        total_spent_endfield += spent;
    }

    let mut total_spent_genshin = 0;
    for _ in 0..simulations {
        let mut genshin_state = GenshinGachaState::default();
        let mut genshin_budget = 100_000;

        genshin_pull_untilrateup_max_pot(&mut genshin_budget, &mut genshin_state, &mut rng);
        let spent = genshin_state.total_pulls;

        total_spent_genshin += spent;
    }

    println!(
        "[ENDFIELD] Average per max pot: {}",
        total_spent_endfield / simulations
    );

    println!(
        "[GENSHIN] Average per max pot: {}",
        total_spent_genshin / simulations
    );
}
