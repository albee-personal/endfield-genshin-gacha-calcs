use rand::{Rng, rngs::ThreadRng};

#[derive(Default, Debug)]
pub struct EndfieldGachaState {
    pub rateups: i32,
    pub offrates: i32,
    pub pity: i32,
    pub guarantee_counter: i32,
    pub total_pulls: i32,
}

const ENDFIELD_BASE_6STAR_RATE: f64 = 0.8 * 0.01; // 0.8%
const ENDFIELD_6STAR_RATEUP: f64 = 0.5; // 50%
const ENDFIELD_PITY_LIMIT: i32 = 120; // 120 pulls

pub fn endfield_pull(state: &mut EndfieldGachaState, rng: &mut ThreadRng) {
    state.total_pulls += 1; // track the pull
    state.guarantee_counter += 1; // track the 120 guarantee

    if state.guarantee_counter >= ENDFIELD_PITY_LIMIT {
        // if we already done 120 pulls, give the 6*
        state.rateups += 1;
        state.guarantee_counter = 0;

        // I assume that the guaranteed 6* DOES reset the pity.
        // this assumption makes the gacha worse than it is, so
        // if I'm wrong then just comment out this and rerun the
        // simulation.
        state.pity = 0;

        return;
    }

    // calculate pity chance
    let mut chance = ENDFIELD_BASE_6STAR_RATE;
    if state.pity > 65 {
        let soft_pity_amount = state.pity as f64 - 65.0;
        chance = chance + 0.05 * soft_pity_amount; // calculate chance accounting for pity
    }

    let got_6star = rng.random_bool(chance);

    if got_6star {
        state.pity = 0;

        // pull a random 6-star
        let got_rateup = rng.random_bool(ENDFIELD_6STAR_RATEUP);

        if got_rateup {
            state.rateups += 1;
        } else {
            state.offrates += 1;
        }
    } else {
        state.pity += 1;
    }
}

pub fn endfield_pull_untilrateup(
    budget: &mut i32,
    state: &mut EndfieldGachaState,
    rng: &mut ThreadRng,
) {
    let previous_rateup_amount = state.rateups;

    // pull until budget runs out.
    for _ in 0..*budget {
        *budget -= 1; // spend 1 pull of budget

        endfield_pull(state, rng);
        if state.rateups > previous_rateup_amount {
            state.guarantee_counter = 0; // reset the guarantee since we stop pulling on the banner once we get a rateup we want
            // we got a rateup! stop pulling.
            return;
        }
    }
}

