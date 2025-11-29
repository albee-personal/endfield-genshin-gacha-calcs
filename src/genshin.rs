use rand::{Rng, rngs::ThreadRng};

#[derive(Default, Debug)]
pub struct GenshinGachaState {
    pub rateups: i32,
    pub offrates: i32,
    pub pity: i32,
    pub has_guarantee: bool,
    pub loss_streak: i32,
    pub total_pulls: i32,
}

// pity data from https://www.hoyolab.com/article/497840
fn genshin_pull(state: &mut GenshinGachaState, rng: &mut ThreadRng) {
    state.total_pulls += 1;

    let mut five_star_probability: f64 = 0.6 * 0.01; // 0.6%

    // soft pity
    if state.pity > 73 {
        let soft_pity_increase = state.pity - 73;
        five_star_probability += soft_pity_increase as f64 * 0.06;

        five_star_probability = five_star_probability.min(1.0); // clamp at 100%
    }

    let got_5star = rng.random_bool(five_star_probability);
    if got_5star || state.pity >= 90 {
        state.pity = 0;

        if state.has_guarantee {
            state.rateups += 1;
            state.has_guarantee = false;

            return;
        }

        // account for capturing radiance
        let rateup_chance = match state.loss_streak {
            2 => 0.75,
            3 => 1.0,
            _ => 0.5,
        };

        let got_rateup = rng.random_bool(rateup_chance);
        if got_rateup {
            state.rateups += 1;
            state.loss_streak = 0; // we won, loss streak is over
            state.has_guarantee = false; // no guarantee after win
        } else {
            state.offrates += 1;
            state.loss_streak += 1; // increment the loss streak
            state.has_guarantee = true; // next one will be guaranteed
        }
    } else {
        state.pity += 1;
    }
}

pub fn genshin_pull_untilrateup(
    budget: &mut i32,
    state: &mut GenshinGachaState,
    rng: &mut ThreadRng,
) {
    let previous_rateup_amount = state.rateups;

    // pull until budget runs out.
    for _ in 0..*budget {
        *budget -= 1; // spend 1 pull of budget

        genshin_pull(state, rng);
        if state.rateups > previous_rateup_amount {
            // we got a rateup! stop pulling.
            return;
        }
    }
}
