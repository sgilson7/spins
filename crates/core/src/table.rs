//! One match of Spins: two sides, their wheels and locks, the three spins of
//! a turn, and the end of the turn worked out as a list of events the views
//! play back.
//!
//! Side 0 is the person (the brief's "User"); side 1 is the computer. Both
//! spin at the same steps; side 1's faces stay inside the core until the
//! reveal (SP-4). End-of-turn order (SP-Q8): both Barricades rise, both sides
//! fill energy and XP, then each side's Legends act, Red then Blue. A side's
//! attacks touch only the other side, so the order between the two sides
//! changes nothing, and both can reach 0 on one turn (a draw, SP-Q9).

use serde::Serialize;

use crate::ai::{self, Public, Skill};
use crate::rng::Rng;
use crate::rules::{self, Grade, Hit, Prof, Sym, Tally, Target, WHEELS};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Legend {
    pub prof: Prof,
    pub energy: i32,
    pub xp: i32,
    pub level: u8,
}

impl Legend {
    pub fn new(prof: Prof) -> Legend {
        Legend {
            prof,
            energy: 0,
            xp: 0,
            level: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Side {
    /// [Red, Blue]
    pub legends: [Legend; 2],
    pub health: i32,
    pub barricade: i32,
    pub faces: [usize; WHEELS],
    pub locks: [bool; WHEELS],
    pub spins_used: u8,
    /// this side has finished spinning for the turn
    pub done: bool,
    pub grade: Grade,
    rng: Rng,
}

impl Side {
    pub fn new(red: Prof, blue: Prof, grade: Grade, seed: u64) -> Side {
        let mut rng = Rng::new(seed);
        let mut faces = [0; WHEELS];
        for f in faces.iter_mut() {
            *f = rng.below(rules::PANELS as u32) as usize;
        }
        Side {
            legends: [Legend::new(red), Legend::new(blue)],
            health: rules::HEALTH,
            barricade: 0,
            faces,
            locks: [false; WHEELS],
            spins_used: 0,
            done: false,
            grade,
            rng,
        }
    }

    pub fn panels(&self) -> [rules::Panel; WHEELS] {
        let mut out = [self.grade.panel(0, 0); WHEELS];
        for (w, o) in out.iter_mut().enumerate() {
            *o = self.grade.panel(w, self.faces[w]);
        }
        out
    }

    pub fn tally(&self) -> Tally {
        rules::tally(&self.panels())
    }

    /// Spins every unlocked wheel. The first spin of a turn turns all five,
    /// because the end of each turn unlocks every wheel.
    fn spin(&mut self) {
        for w in 0..WHEELS {
            if !self.locks[w] {
                self.faces[w] = self.rng.below(rules::PANELS as u32) as usize;
            }
        }
        self.spins_used += 1;
        if self.spins_used >= rules::SPINS {
            self.done = true;
        }
    }

    pub fn all_locked(&self) -> bool {
        self.locks.iter().all(|&l| l)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Outcome {
    Win,
    Loss,
    Draw,
}

/// One step of the end of a turn. Each carries the numbers after it, so a
/// view can show them as it plays the list without working anything out.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Event {
    Reveal {
        faces: [[usize; WHEELS]; 2],
    },
    Count {
        side: usize,
        sym: Sym,
        symbols: i32,
        segments: i32,
    },
    Barricade {
        side: usize,
        value: i32,
        added: i32,
    },
    Energy {
        side: usize,
        slot: usize,
        value: i32,
        added: i32,
    },
    Xp {
        side: usize,
        slot: usize,
        value: i32,
        added: i32,
    },
    Evolve {
        side: usize,
        slot: usize,
        level: u8,
    },
    /// `foe` is the side that took it
    Burst {
        side: usize,
        slot: usize,
        foe: usize,
        amount: i32,
        health: i32,
    },
    /// `energy` is the bar after acting
    Act {
        side: usize,
        slot: usize,
        prof: Prof,
        energy: i32,
    },
    Hit {
        side: usize,
        slot: usize,
        foe: usize,
        target: Target,
        amount: i32,
        over: bool,
        barricade: i32,
        health: i32,
    },
    Over {
        outcome: Outcome,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct Stats {
    /// times each Legend acted: [side][slot]
    pub acts: [[i32; 2]; 2],
    pub damage: [i32; 2],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Match {
    pub sides: [Side; 2],
    pub turn: u32,
    pub skill: Skill,
    pub outcome: Option<Outcome>,
    pub stats: Stats,
    ai_rng: Rng,
}

/// Seeds for the two sides' wheels and the opponent's thinking, each its own
/// stream, so the opponent's sampling never moves where a wheel lands.
fn streams(seed: u64) -> [u64; 3] {
    let mut r = Rng::new(seed);
    [r.next_u64(), r.next_u64(), r.next_u64()]
}

pub struct Setup {
    pub you: [Prof; 2],
    pub you_grade: Grade,
    pub them: [Prof; 2],
    pub them_grade: Grade,
    pub skill: Skill,
}

impl Match {
    pub fn new(setup: &Setup, seed: u64) -> Match {
        let [a, b, c] = streams(seed);
        Match {
            sides: [
                Side::new(setup.you[0], setup.you[1], setup.you_grade, a),
                Side::new(setup.them[0], setup.them[1], setup.them_grade, b),
            ],
            turn: 1,
            skill: setup.skill,
            outcome: None,
            stats: Stats::default(),
            ai_rng: Rng::new(c),
        }
    }

    pub fn can_spin(&self) -> bool {
        let s = &self.sides[0];
        self.outcome.is_none() && !s.done && (s.spins_used == 0 || !s.all_locked())
    }

    pub fn can_lock(&self) -> bool {
        let s = &self.sides[0];
        self.outcome.is_none() && s.spins_used >= 1 && !s.done
    }

    pub fn can_end(&self) -> bool {
        self.can_lock()
    }

    pub fn toggle_lock(&mut self, wheel: usize) -> Result<(), String> {
        if !self.can_lock() {
            return Err("Locks change between spins.".into());
        }
        let l = self.sides[0]
            .locks
            .get_mut(wheel)
            .ok_or("There is no such wheel.")?;
        *l = !*l;
        Ok(())
    }

    /// Your spin; the opponent takes its own step at the same time. After
    /// the third spin the turn ends and the events come back.
    pub fn spin(&mut self) -> Result<Vec<Event>, String> {
        if !self.can_spin() {
            return Err("No spin is left this turn.".into());
        }
        self.sides[0].spin();
        self.opponent_step();
        if self.sides[0].done {
            return Ok(self.finish_turn());
        }
        Ok(Vec::new())
    }

    /// Ends your turn with what is showing; the opponent finishes its spins.
    pub fn end_turn(&mut self) -> Result<Vec<Event>, String> {
        if !self.can_end() {
            return Err("Spin once before ending the turn.".into());
        }
        self.sides[0].done = true;
        Ok(self.finish_turn())
    }

    /// The opponent's step: it reads only its own side and the public part
    /// of yours (SP-3).
    fn opponent_step(&mut self) {
        let public = Public::of(&self.sides[0]);
        let me = &mut self.sides[1];
        if me.done {
            return;
        }
        if me.spins_used > 0 {
            let d = ai::decide(self.skill, me, &public, &mut self.ai_rng);
            if d.stop || d.locks.iter().all(|&l| l) {
                me.done = true;
                return;
            }
            me.locks = d.locks;
        }
        me.spin();
    }

    fn finish_turn(&mut self) -> Vec<Event> {
        while !self.sides[1].done {
            self.opponent_step();
        }
        let mut ev = vec![Event::Reveal {
            faces: [self.sides[0].faces, self.sides[1].faces],
        }];
        let tallies = [self.sides[0].tally(), self.sides[1].tally()];
        for (side, t) in tallies.iter().enumerate() {
            for (sym, n) in [
                (Sym::Red, t.red),
                (Sym::Blue, t.blue),
                (Sym::Barricade, t.barricade),
            ] {
                let segments = rules::segments(n);
                if segments > 0 {
                    ev.push(Event::Count {
                        side,
                        sym,
                        symbols: n,
                        segments,
                    });
                }
            }
        }
        // Barricades rise first (SP-Q8); anything above the top is dropped.
        for (side, t) in tallies.iter().enumerate() {
            let s = &mut self.sides[side];
            let before = s.barricade;
            s.barricade = (s.barricade + rules::segments(t.barricade)).min(rules::BARRICADE_MAX);
            if s.barricade != before {
                ev.push(Event::Barricade {
                    side,
                    value: s.barricade,
                    added: s.barricade - before,
                });
            }
        }
        // Energy, then XP from gilded panels; energy past full is dropped (SP-Q7).
        for (side, t) in tallies.iter().enumerate() {
            for (slot, segs) in [rules::segments(t.red), rules::segments(t.blue)]
                .into_iter()
                .enumerate()
            {
                let l = &mut self.sides[side].legends[slot];
                let before = l.energy;
                l.energy = (l.energy + segs).min(l.prof.cost());
                if l.energy != before {
                    ev.push(Event::Energy {
                        side,
                        slot,
                        value: l.energy,
                        added: l.energy - before,
                    });
                }
            }
            for (slot, n) in [t.red_gilded, t.blue_gilded].into_iter().enumerate() {
                if n > 0 {
                    self.gain_xp(side, slot, n, &mut ev);
                }
            }
        }
        // Full Legends act, Red then Blue, yours then theirs.
        for side in 0..2 {
            for slot in 0..2 {
                let l = &self.sides[side].legends[slot];
                if l.energy < l.prof.cost() {
                    continue;
                }
                let (prof, level) = (l.prof, l.level);
                ev.push(Event::Act {
                    side,
                    slot,
                    prof,
                    energy: 0,
                });
                let foe = &mut self.sides[1 - side];
                let hits: Vec<Hit> = rules::attack(prof, level, &mut foe.barricade, &mut foe.health);
                let (barricade, health) = (foe.barricade, foe.health);
                for h in hits {
                    if h.target == Target::Health {
                        self.stats.damage[side] += h.amount;
                    }
                    ev.push(Event::Hit {
                        side,
                        slot,
                        foe: 1 - side,
                        target: h.target,
                        amount: h.amount,
                        over: h.over,
                        barricade,
                        health,
                    });
                }
                self.sides[side].legends[slot].energy = 0;
                self.stats.acts[side][slot] += 1;
                self.gain_xp(side, slot, rules::XP_PER_ACTION, &mut ev);
            }
        }
        let (you, them) = (self.sides[0].health, self.sides[1].health);
        self.outcome = match (you == 0, them == 0) {
            (true, true) => Some(Outcome::Draw),
            (true, false) => Some(Outcome::Loss),
            (false, true) => Some(Outcome::Win),
            (false, false) => None,
        };
        if let Some(outcome) = self.outcome {
            ev.push(Event::Over { outcome });
        } else {
            self.turn += 1;
            for s in self.sides.iter_mut() {
                s.spins_used = 0;
                s.done = false;
                s.locks = [false; WHEELS];
            }
        }
        ev
    }

    /// XP past the bar carries into the next level. A Gold Legend that fills
    /// its bar again does damage straight to the other side's health.
    fn gain_xp(&mut self, side: usize, slot: usize, n: i32, ev: &mut Vec<Event>) {
        let l = &mut self.sides[side].legends[slot];
        l.xp += n;
        let mut bursts = 0;
        let mut evolved = Vec::new();
        while l.xp >= rules::XP_MAX {
            l.xp -= rules::XP_MAX;
            if l.level < rules::LEVEL_MAX {
                l.level += 1;
                evolved.push(l.level);
            } else {
                bursts += 1;
            }
        }
        ev.push(Event::Xp {
            side,
            slot,
            value: l.xp,
            added: n,
        });
        for level in evolved {
            ev.push(Event::Evolve { side, slot, level });
        }
        for _ in 0..bursts {
            let foe = &mut self.sides[1 - side];
            let amount = rules::GOLD_BURST.min(foe.health);
            foe.health -= amount;
            self.stats.damage[side] += amount;
            ev.push(Event::Burst {
                side,
                slot,
                foe: 1 - side,
                amount,
                health: foe.health,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Setup {
        Setup {
            you: [Prof::Knight, Prof::Wizard],
            you_grade: Grade::Pine,
            them: [Prof::Bowman, Prof::Knight],
            them_grade: Grade::Pine,
            skill: Skill::Fair,
        }
    }

    /// The face on `wheel` that shows `kind`, `count` and `gilded`.
    fn face(grade: Grade, wheel: usize, sym: Sym, count: u8, gilded: bool) -> usize {
        (0..rules::PANELS)
            .find(|&f| {
                let p = grade.panel(wheel, f);
                p.sym == sym && p.count == count && p.gilded == gilded
            })
            .unwrap()
    }

    fn set_faces(s: &mut Side, want: [(Sym, u8, bool); WHEELS]) {
        for (w, (sym, n, g)) in want.into_iter().enumerate() {
            s.faces[w] = face(s.grade, w, sym, n, g);
        }
    }

    #[test]
    fn a_turn_is_three_spins_then_the_end_of_the_turn() {
        let mut m = Match::new(&setup(), 7);
        assert!(m.spin().unwrap().is_empty());
        assert!(m.spin().unwrap().is_empty());
        let ev = m.spin().unwrap();
        assert!(matches!(ev[0], Event::Reveal { .. }));
        assert_eq!(m.turn, 2);
        assert_eq!(m.sides[0].spins_used, 0);
    }

    #[test]
    fn locks_wait_for_the_first_spin_and_hold_a_wheel_still() {
        let mut m = Match::new(&setup(), 3);
        assert!(m.toggle_lock(0).is_err());
        m.spin().unwrap();
        m.toggle_lock(2).unwrap();
        let held = m.sides[0].faces[2];
        for _ in 0..1 {
            m.spin().unwrap();
        }
        assert_eq!(m.sides[0].faces[2], held);
        // a lock can be undone before the last spin
        m.toggle_lock(2).unwrap();
        assert!(!m.sides[0].locks[2]);
    }

    #[test]
    fn five_locks_leave_no_spin_but_the_turn_can_end() {
        let mut m = Match::new(&setup(), 5);
        m.spin().unwrap();
        for w in 0..WHEELS {
            m.toggle_lock(w).unwrap();
        }
        assert!(!m.can_spin());
        assert!(m.spin().is_err());
        assert!(matches!(m.end_turn().unwrap()[0], Event::Reveal { .. }));
    }

    #[test]
    fn the_turn_cannot_end_before_the_first_spin() {
        let mut m = Match::new(&setup(), 5);
        assert!(m.end_turn().is_err());
    }

    #[test]
    fn the_end_of_a_turn_builds_fills_and_attacks_in_order() {
        use Sym::*;
        let mut m = Match::new(&setup(), 11);
        m.spin().unwrap();
        // you: red 1+2+1 = 4 (2 segments), Barricade 2+1 = 3 (1 segment), one red gilded
        set_faces(
            &mut m.sides[0],
            [
                (Red, 1, false),
                (Red, 2, false),
                (Red, 1, true),
                (Barricade, 2, false),
                (Barricade, 1, false),
            ],
        );
        m.sides[0].legends[0].energy = 1; // the Knight needs 3: 1 + 2 fills it
        m.sides[1].barricade = 0;
        m.sides[1].done = true; // the opponent keeps what it shows
        set_faces(&mut m.sides[1], [(Blue, 1, false); WHEELS]); // blue 5: 3 segments
        let ev = m.end_turn().unwrap();
        assert!(ev.contains(&Event::Barricade {
            side: 0,
            value: 1,
            added: 1
        }));
        assert_eq!(
            m.sides[1].health, 7,
            "the Knight's 3 lands on health: the opponent had no Barricade"
        );
        assert_eq!(m.sides[0].legends[0].energy, 0, "the bar empties after acting");
        // XP: 1 from the gilded panel, 2 for acting
        assert_eq!(m.sides[0].legends[0].xp, 3);
        // the opponent's Knight sits Blue and needs 3: 3 segments fill it; your Barricade 1 takes it
        assert_eq!(m.sides[0].health, 10);
        assert_eq!(m.sides[0].barricade, 0);
        let kinds: Vec<&str> = ev
            .iter()
            .map(|e| match e {
                Event::Reveal { .. } => "reveal",
                Event::Count { .. } => "count",
                Event::Barricade { .. } => "barricade",
                Event::Energy { .. } => "energy",
                Event::Xp { .. } => "xp",
                Event::Act { .. } => "act",
                Event::Hit { .. } => "hit",
                _ => "other",
            })
            .collect();
        assert_eq!(
            kinds,
            [
                "reveal",
                "count",
                "count",
                "count",
                "barricade",
                "energy",
                "xp",
                "energy",
                "act",
                "hit",
                "xp",
                "act",
                "hit",
                "xp"
            ]
        );
    }

    #[test]
    fn energy_past_a_full_bar_is_dropped() {
        use Sym::*;
        let mut m = Match::new(&setup(), 2);
        m.spin().unwrap();
        set_faces(&mut m.sides[0], [(Red, 2, false); WHEELS]); // red 10: 8 segments
        m.sides[1].done = true;
        m.sides[1].barricade = 5;
        let ev = m.end_turn().unwrap();
        // 8 segments into a bar of 3: it fills to 3, and the other 5 are dropped
        assert!(
            ev.contains(&Event::Energy {
                side: 0,
                slot: 0,
                value: 3,
                added: 3
            }),
            "{ev:?}"
        );
        assert_eq!(m.sides[0].legends[0].energy, 0, "and it empties after acting");
    }

    #[test]
    fn a_new_turn_starts_with_every_wheel_unlocked() {
        let mut m = Match::new(&setup(), 9);
        m.spin().unwrap();
        m.toggle_lock(1).unwrap();
        m.toggle_lock(3).unwrap();
        m.end_turn().unwrap();
        assert_eq!(m.sides[0].locks, [false; WHEELS]);
        let held = m.sides[0].faces;
        // over many seeds' worth of spins, every wheel moves on the first spin of the turn
        let mut moved = [false; WHEELS];
        for _ in 0..20 {
            if m.outcome.is_some() {
                break;
            }
            m.spin().unwrap();
            for w in 0..WHEELS {
                moved[w] |= m.sides[0].faces[w] != held[w];
            }
            if m.outcome.is_some() {
                break;
            }
            m.end_turn().unwrap();
        }
        assert_eq!(moved, [true; WHEELS]);
    }

    #[test]
    fn a_barricade_stops_at_five() {
        use Sym::*;
        let mut m = Match::new(&setup(), 2);
        m.spin().unwrap();
        m.sides[0].barricade = 4;
        set_faces(&mut m.sides[0], [(Barricade, 2, false); WHEELS]); // 10 symbols: 8 segments
        m.sides[1].done = true;
        set_faces(&mut m.sides[1], [(Barricade, 1, false); WHEELS]);
        m.end_turn().unwrap();
        assert_eq!(m.sides[0].barricade, 5);
    }

    #[test]
    fn six_xp_evolves_a_legend_and_a_gold_legend_bursts() {
        let mut m = Match::new(&setup(), 2);
        let mut ev = Vec::new();
        m.gain_xp(0, 0, 6, &mut ev);
        assert_eq!((m.sides[0].legends[0].level, m.sides[0].legends[0].xp), (1, 0));
        m.gain_xp(0, 0, 7, &mut ev);
        assert_eq!((m.sides[0].legends[0].level, m.sides[0].legends[0].xp), (2, 1));
        m.gain_xp(0, 0, 5, &mut ev);
        assert_eq!(m.sides[1].health, 8, "Gold and full again: 2 straight to health");
        assert_eq!(m.sides[0].legends[0].xp, 0);
    }

    #[test]
    fn both_at_zero_on_one_turn_is_a_draw() {
        use Sym::*;
        let mut m = Match::new(&setup(), 2);
        m.spin().unwrap();
        m.sides[0].health = 3;
        m.sides[1].health = 3;
        m.sides[0].legends[0].energy = 2;
        m.sides[1].legends[1].energy = 2; // their Knight
        set_faces(
            &mut m.sides[0],
            [
                (Red, 1, false),
                (Red, 1, false),
                (Red, 1, false),
                (Blue, 1, false),
                (Blue, 1, false),
            ],
        );
        m.sides[1].done = true;
        set_faces(
            &mut m.sides[1],
            [
                (Blue, 1, false),
                (Blue, 1, false),
                (Blue, 1, false),
                (Red, 1, false),
                (Red, 1, false),
            ],
        );
        let ev = m.end_turn().unwrap();
        assert_eq!(m.outcome, Some(Outcome::Draw));
        assert!(matches!(
            ev.last(),
            Some(Event::Over {
                outcome: Outcome::Draw
            })
        ));
    }

    #[test]
    fn the_same_seed_and_choices_give_the_same_match() {
        let play = |seed| {
            let mut m = Match::new(&setup(), seed);
            let mut log = Vec::new();
            while m.outcome.is_none() && m.turn < 60 {
                m.spin().unwrap();
                m.toggle_lock(0).unwrap();
                m.spin().unwrap();
                log.extend(m.spin().unwrap());
            }
            serde_json::to_string(&log).unwrap()
        };
        assert_eq!(play(42), play(42));
        assert_ne!(play(42), play(43));
    }
}
