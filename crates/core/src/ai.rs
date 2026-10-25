//! The computer opponent. It decides which wheels to lock, and when to stop
//! spinning, from its own side and the public part of yours: health,
//! Barricade and the Legends' bars. Your faces are not in `Public`, so it
//! cannot use them (SP-3).

use serde::{Deserialize, Serialize};

use crate::rng::Rng;
use crate::rules::{self, Prof, Sym, Tally, WHEELS};
use crate::table::Side;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Skill {
    Easy,
    Fair,
    Hard,
}

/// What anyone at the table can see of a side during a turn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Public {
    pub health: i32,
    pub barricade: i32,
    /// (proficiency, energy, level) for Red and Blue
    pub legends: [(Prof, i32, u8); 2],
}

impl Public {
    pub fn of(s: &Side) -> Public {
        let l = |i: usize| (s.legends[i].prof, s.legends[i].energy, s.legends[i].level);
        Public {
            health: s.health,
            barricade: s.barricade,
            legends: [l(0), l(1)],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decision {
    pub locks: [bool; WHEELS],
    pub stop: bool,
}

/// Samples per lock choice for the hard opponent.
const SAMPLES: u32 = 24;

pub fn decide(skill: Skill, me: &Side, you: &Public, rng: &mut Rng) -> Decision {
    match skill {
        Skill::Easy => easy(me, rng),
        Skill::Fair => fair(me, you),
        Skill::Hard => hard(me, you, rng),
    }
}

fn kind_of(me: &Side, w: usize) -> Sym {
    me.grade.panel(w, me.faces[w]).sym
}

fn symbols(t: &Tally, sym: Sym) -> i32 {
    match sym {
        Sym::Red => t.red,
        Sym::Blue => t.blue,
        Sym::Barricade => t.barricade,
    }
}

fn lock_kind(me: &Side, sym: Sym) -> [bool; WHEELS] {
    let mut locks = [false; WHEELS];
    for (w, l) in locks.iter_mut().enumerate() {
        *l = kind_of(me, w) == sym;
    }
    locks
}

/// Locks whatever kind shows most, and misses a wheel one time in four.
fn easy(me: &Side, rng: &mut Rng) -> Decision {
    let t = me.tally();
    let best = [Sym::Red, Sym::Blue, Sym::Barricade]
        .into_iter()
        .max_by_key(|&s| symbols(&t, s))
        .unwrap_or(Sym::Red);
    let mut locks = lock_kind(me, best);
    for l in locks.iter_mut() {
        if *l && rng.chance(1, 4) {
            *l = false;
        }
    }
    Decision { locks, stop: false }
}

/// The damage your full Legends would do next turn, a measure of how much a
/// Barricade is worth now.
fn threat(you: &Public, my_barricade: i32) -> i32 {
    let mut total = 0;
    for &(prof, energy, level) in &you.legends {
        if energy + 2 >= prof.cost() {
            let (mut b, mut h) = (my_barricade, rules::HEALTH);
            rules::attack(prof, level, &mut b, &mut h);
            total += rules::HEALTH - h;
        }
    }
    total
}

/// How good a set of faces is for this side, in points: an attack that
/// fires is worth most, progress on a bar less, a Barricade more when you
/// threaten, XP a little.
pub fn score(me: &Side, you: &Public, t: &Tally) -> i32 {
    let mut s = 0;
    for (slot, n) in [t.red, t.blue].into_iter().enumerate() {
        let l = &me.legends[slot];
        let cost = l.prof.cost();
        let after = (l.energy + rules::segments(n)).min(cost);
        if after == cost {
            let (mut b, mut h) = (you.barricade, you.health);
            let hits = rules::attack(l.prof, l.level, &mut b, &mut h);
            let dmg: i32 = hits
                .iter()
                .map(|x| {
                    if x.target == rules::Target::Health {
                        3 * x.amount
                    } else {
                        x.amount
                    }
                })
                .sum();
            s += 40 + 6 * dmg;
        } else {
            s += 12 * (after - l.energy) + 2 * n.min(rules::OAK);
        }
    }
    let room = rules::BARRICADE_MAX - me.barricade;
    let built = rules::segments(t.barricade).min(room);
    s += built * (6 + 4 * threat(you, me.barricade)) + 2 * t.barricade.min(rules::OAK);
    s + 2 * (t.red_gilded + t.blue_gilded)
}

/// Chases one goal: the bar nearest to full, or the Barricade when your
/// Legends threaten. Stops once the goal is met.
fn fair(me: &Side, you: &Public) -> Decision {
    let t = me.tally();
    let need = |slot: usize| me.legends[slot].prof.cost() - me.legends[slot].energy;
    let mut goal = if need(0) <= need(1) { Sym::Red } else { Sym::Blue };
    if threat(you, me.barricade) >= 3 && me.barricade < rules::BARRICADE_MAX {
        goal = Sym::Barricade;
    }
    let have = rules::segments(symbols(&t, goal));
    let met = match goal {
        Sym::Red => have >= need(0),
        Sym::Blue => have >= need(1),
        Sym::Barricade => have >= 2,
    };
    Decision {
        locks: lock_kind(me, goal),
        stop: met,
    }
}

/// Tries all 32 ways to lock, rerolls the unlocked wheels from samples, and
/// keeps the choice with the best average score. Stopping is a candidate too.
fn hard(me: &Side, you: &Public, rng: &mut Rng) -> Decision {
    let now = score(me, you, &me.tally());
    let mut best = (
        now * SAMPLES as i32,
        Decision {
            locks: [true; WHEELS],
            stop: true,
        },
    );
    for mask in 0..(1u32 << WHEELS) {
        let mut locks = [false; WHEELS];
        for (w, l) in locks.iter_mut().enumerate() {
            *l = mask & (1 << w) != 0;
        }
        if locks.iter().all(|&l| l) {
            continue;
        }
        let mut total = 0;
        for _ in 0..SAMPLES {
            let mut panels = me.panels();
            for (w, p) in panels.iter_mut().enumerate() {
                if !locks[w] {
                    *p = me.grade.panel(w, rng.below(rules::PANELS as u32) as usize);
                }
            }
            total += score(me, you, &rules::tally(&panels));
        }
        if total > best.0 {
            best = (total, Decision { locks, stop: false });
        }
    }
    best.1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::Grade;

    fn side(seed: u64) -> Side {
        Side::new(Prof::Knight, Prof::Wizard, Grade::Pine, seed)
    }

    #[test]
    fn what_the_opponent_sees_of_you_holds_no_faces() {
        let mut a = side(1);
        let mut b = a.clone();
        a.faces = [0, 0, 0, 0, 0];
        b.faces = [9, 8, 7, 6, 5];
        a.locks = [true; WHEELS];
        assert_eq!(Public::of(&a), Public::of(&b));
    }

    #[test]
    fn every_skill_decides_the_same_whatever_your_faces() {
        let me = side(9);
        for skill in [Skill::Easy, Skill::Fair, Skill::Hard] {
            let mut you_a = side(2);
            let mut you_b = you_a.clone();
            you_a.faces = [1, 2, 3, 4, 5];
            you_b.faces = [5, 4, 3, 2, 1];
            let d_a = decide(skill, &me, &Public::of(&you_a), &mut Rng::new(4));
            let d_b = decide(skill, &me, &Public::of(&you_b), &mut Rng::new(4));
            assert_eq!(d_a, d_b);
            you_a.faces = [0; WHEELS];
        }
    }

    #[test]
    fn fair_locks_the_reds_when_its_red_legend_is_closest() {
        let mut me = side(3);
        me.legends[0].energy = 2; // Knight needs 1
                                  // wheel 0 and 1 red, the rest blue
        let face = |w: usize, sym: Sym| (0..10).find(|&f| Grade::Pine.panel(w, f).sym == sym).unwrap();
        me.faces = [
            face(0, Sym::Red),
            face(1, Sym::Red),
            face(2, Sym::Blue),
            face(3, Sym::Blue),
            face(4, Sym::Blue),
        ];
        let you = Public::of(&side(4));
        let d = fair(&me, &you);
        assert_eq!(d.locks, [true, true, false, false, false]);
    }

    #[test]
    fn a_firing_attack_scores_above_any_progress() {
        let mut me = side(3);
        me.legends[0].energy = 2;
        let you = Public::of(&side(4));
        let fires = Tally {
            red: 3,
            ..Tally::default()
        };
        let progress = Tally {
            blue: 6,
            ..Tally::default()
        };
        assert!(score(&me, &you, &fires) > score(&me, &you, &progress));
    }

    #[test]
    fn the_hard_opponent_beats_the_easy_one_over_many_matches() {
        use crate::table::{Match, Setup};
        let mut hard_wins = 0;
        let mut easy_wins = 0;
        for seed in 0..60 {
            // the person's side plays as the easy opponent here: locks its most common kind
            let mut m = Match::new(
                &Setup {
                    you: [Prof::Knight, Prof::Wizard],
                    you_grade: Grade::Pine,
                    them: [Prof::Knight, Prof::Wizard],
                    them_grade: Grade::Pine,
                    skill: Skill::Hard,
                },
                seed,
            );
            let mut r = Rng::new(seed + 1000);
            while m.outcome.is_none() {
                m.spin().unwrap();
                for _ in 0..2 {
                    if m.outcome.is_some() || !m.can_lock() {
                        break;
                    }
                    let d = easy(&m.sides[0], &mut r);
                    m.sides[0].locks = d.locks;
                    if m.sides[0].all_locked() {
                        m.end_turn().unwrap();
                    } else {
                        m.spin().unwrap();
                    }
                }
            }
            match m.outcome {
                Some(crate::table::Outcome::Loss) => hard_wins += 1,
                Some(crate::table::Outcome::Win) => easy_wins += 1,
                _ => {}
            }
        }
        assert!(hard_wins > easy_wins, "hard {hard_wins} easy {easy_wins}");
    }
}
