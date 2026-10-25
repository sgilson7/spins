//! The balance run promised under SP-Q14: computer against computer on the
//! person's wheels, reporting how long matches last. Run with
//! `cargo test --test balance -- --nocapture` to see the numbers.

use tool_core::ai::{self, Public, Skill};
use tool_core::rng::Rng;
use tool_core::rules::{Grade, Prof};
use tool_core::table::{Match, Outcome, Setup};

fn play(
    seed: u64,
    you: Skill,
    them: Skill,
    legends: [[Prof; 2]; 2],
    grades: [Grade; 2],
) -> (u32, Option<Outcome>) {
    let mut m = Match::new(
        &Setup {
            you: legends[0],
            you_grade: grades[0],
            them: legends[1],
            them_grade: grades[1],
            skill: them,
        },
        seed,
    );
    let mut r = Rng::new(seed ^ 0x5eed);
    while m.outcome.is_none() && m.turn < 200 {
        m.spin().unwrap();
        while m.outcome.is_none() && m.can_lock() {
            let d = ai::decide(you, &m.sides[0], &Public::of(&m.sides[1]), &mut r);
            m.sides[0].locks = d.locks;
            if d.stop || m.sides[0].all_locked() {
                m.end_turn().unwrap();
            } else {
                m.spin().unwrap();
            }
        }
    }
    (m.turn, m.outcome)
}

fn report(label: &str, you: Skill, them: Skill, legends: [[Prof; 2]; 2], grades: [Grade; 2]) -> (i32, i32) {
    let n = 1000;
    let (mut turns, mut wins, mut losses, mut draws, mut longest) = (0, 0, 0, 0, 0);
    for seed in 0..n {
        let (t, o) = play(seed, you, them, legends, grades);
        turns += t;
        longest = longest.max(t);
        match o {
            Some(Outcome::Win) => wins += 1,
            Some(Outcome::Loss) => losses += 1,
            _ => draws += 1,
        }
    }
    let mean10 = (turns * 10 / n as u32) as i32;
    println!(
        "{label}: mean {}.{} turns, longest {longest}; side 0 won {wins}, lost {losses}, drew {draws}",
        mean10 / 10,
        mean10 % 10
    );
    (mean10, wins)
}

#[test]
fn matches_last_a_handful_of_turns_and_skill_tells() {
    use Prof::*;
    let kw = [Knight, Wizard];
    let (mean, _) = report(
        "fair v fair, Knight+Wizard both",
        Skill::Fair,
        Skill::Fair,
        [kw, kw],
        [Grade::Pine; 2],
    );
    report(
        "hard v hard",
        Skill::Hard,
        Skill::Hard,
        [kw, kw],
        [Grade::Pine; 2],
    );
    let (_, hard_wins) = report(
        "hard v easy",
        Skill::Hard,
        Skill::Easy,
        [kw, kw],
        [Grade::Pine; 2],
    );
    report(
        "fair v hard",
        Skill::Fair,
        Skill::Hard,
        [kw, kw],
        [Grade::Pine; 2],
    );
    report(
        "fair pine v fair oak",
        Skill::Fair,
        Skill::Fair,
        [kw, kw],
        [Grade::Pine, Grade::Oak],
    );
    report(
        "Knight+Bowman v Wizard+Bowman, fair",
        Skill::Fair,
        Skill::Fair,
        [[Knight, Bowman], [Wizard, Bowman]],
        [Grade::Pine; 2],
    );
    report(
        "hard v Halvard (Bowman+Knight, oak)",
        Skill::Hard,
        Skill::Hard,
        [kw, [Bowman, Knight]],
        [Grade::Pine, Grade::Oak],
    );
    assert!(
        (30..=200).contains(&mean),
        "a match should last about 3 to 20 turns, mean x10 = {mean}"
    );
    assert!(
        hard_wins > 600,
        "the hard opponent should beat the easy one most of the time: {hard_wins}"
    );
}
