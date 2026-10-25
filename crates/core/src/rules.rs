//! The rules of Spins that do not depend on a match in progress: the
//! symbols, the panels and the wheels they sit on, the 3oak+ count, and what
//! each proficiency does when its energy bar is full.
//!
//! Brief words are used for names: a panel shows 1 or 2 symbols of one kind
//! (red energy, blue energy or Barricade); a gilded panel also gives XP
//! (Sea of Stars' blue-backed tiles, SP-Q5).

use serde::Serialize;

pub const WHEELS: usize = 5;
pub const PANELS: usize = 10;
pub const SPINS: u8 = 3;
pub const HEALTH: i32 = 10;
pub const BARRICADE_MAX: i32 = 5;
/// XP a Legend needs to evolve (Sea of Stars: six notches).
pub const XP_MAX: i32 = 6;
/// XP a Legend gains each time it acts.
pub const XP_PER_ACTION: i32 = 2;
/// Damage straight to health when a Gold Legend fills its XP again.
pub const GOLD_BURST: i32 = 2;
/// Bronze, Silver, Gold.
pub const LEVEL_MAX: u8 = 2;
/// The Bowman's flight height: a Barricade lower than this is passed.
pub const BOWMAN_HEIGHT: i32 = 3;
/// Symbols of one kind that make the first segment (3oak+).
pub const OAK: i32 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Sym {
    Red,
    Blue,
    Barricade,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Panel {
    pub sym: Sym,
    pub count: u8,
    pub gilded: bool,
}

const fn p(sym: Sym, count: u8, gilded: bool) -> Panel {
    Panel { sym, count, gilded }
}

/// The ten panels every wheel carries, in the person's list (T2, from
/// SP-Q14): red ×1, red ×1 gilded, red ×2, red ×2 gilded, blue ×1, blue ×1
/// gilded, blue ×2, blue ×2 gilded, Barricade ×1, Barricade ×2.
pub const PINE: [Panel; PANELS] = [
    p(Sym::Red, 1, false),
    p(Sym::Red, 1, true),
    p(Sym::Red, 2, false),
    p(Sym::Red, 2, true),
    p(Sym::Blue, 1, false),
    p(Sym::Blue, 1, true),
    p(Sym::Blue, 2, false),
    p(Sym::Blue, 2, true),
    p(Sym::Barricade, 1, false),
    p(Sym::Barricade, 2, false),
];

/// Oak wheels (T4, from SP-Q18): a better grade carries more doubles. The
/// plain singles become doubles; the gilded singles and the Barricade single
/// stay.
pub const OAK_WOOD: [Panel; PANELS] = [
    p(Sym::Red, 2, false),
    p(Sym::Red, 1, true),
    p(Sym::Red, 2, false),
    p(Sym::Red, 2, true),
    p(Sym::Blue, 2, false),
    p(Sym::Blue, 1, true),
    p(Sym::Blue, 2, false),
    p(Sym::Blue, 2, true),
    p(Sym::Barricade, 1, false),
    p(Sym::Barricade, 2, false),
];

/// Where each of the ten panels sits around each wheel: the same panels on
/// every wheel, in a different order, so neighbours differ from wheel to
/// wheel.
pub const ORDER: [[usize; PANELS]; WHEELS] = [
    [0, 6, 8, 3, 4, 9, 2, 5, 1, 7],
    [4, 2, 9, 5, 0, 8, 6, 1, 7, 3],
    [8, 1, 4, 2, 9, 7, 0, 6, 3, 5],
    [2, 5, 8, 4, 3, 9, 7, 0, 1, 6],
    [6, 0, 9, 1, 7, 8, 2, 4, 5, 3],
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Grade {
    Pine,
    Oak,
}

impl Grade {
    pub fn key(self) -> &'static str {
        match self {
            Grade::Pine => "pine",
            Grade::Oak => "oak",
        }
    }
    pub fn panels(self) -> &'static [Panel; PANELS] {
        match self {
            Grade::Pine => &PINE,
            Grade::Oak => &OAK_WOOD,
        }
    }
    /// The panel at position `face` of wheel `wheel`.
    pub fn panel(self, wheel: usize, face: usize) -> Panel {
        self.panels()[ORDER[wheel][face]]
    }
}

/// Symbols of each kind showing on five faces: (red, blue, Barricade), and
/// the gilded panels of each energy colour: (red, blue).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
pub struct Tally {
    pub red: i32,
    pub blue: i32,
    pub barricade: i32,
    pub red_gilded: i32,
    pub blue_gilded: i32,
}

pub fn tally(panels: &[Panel]) -> Tally {
    let mut t = Tally::default();
    for p in panels {
        let n = p.count as i32;
        match p.sym {
            Sym::Red => {
                t.red += n;
                t.red_gilded += p.gilded as i32;
            }
            Sym::Blue => {
                t.blue += n;
                t.blue_gilded += p.gilded as i32;
            }
            Sym::Barricade => t.barricade += n,
        }
    }
    t
}

/// 3oak+: three symbols of a kind make one segment, and each symbol after
/// the third makes one more.
pub fn segments(symbols: i32) -> i32 {
    if symbols >= OAK {
        symbols - (OAK - 1)
    } else {
        0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Prof {
    Knight,
    Wizard,
    Bowman,
}

impl Prof {
    pub const ALL: [Prof; 3] = [Prof::Knight, Prof::Wizard, Prof::Bowman];

    pub fn key(self) -> &'static str {
        match self {
            Prof::Knight => "knight",
            Prof::Wizard => "wizard",
            Prof::Bowman => "bowman",
        }
    }

    pub fn from_key(key: &str) -> Option<Prof> {
        Prof::ALL.into_iter().find(|p| p.key() == key)
    }

    /// Energy segments the bar holds; a full bar acts.
    pub fn cost(self) -> i32 {
        match self {
            Prof::Knight => 3,
            Prof::Wizard => 5,
            Prof::Bowman => 4,
        }
    }

    /// The damage numbers at Bronze: (to health, to the Barricade). Each
    /// level adds one to both (SP-Q6).
    fn base(self) -> (i32, i32) {
        match self {
            Prof::Knight => (3, 3),
            Prof::Wizard => (2, 2),
            Prof::Bowman => (3, 1),
        }
    }

    pub fn damage(self, level: u8) -> (i32, i32) {
        let (h, b) = self.base();
        (h + level as i32, b + level as i32)
    }
}

/// Where one attack landed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Target {
    Barricade,
    Health,
}

/// One attack: where it landed, how much it took, and whether it flew over
/// a standing Barricade to get there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Hit {
    pub target: Target,
    pub amount: i32,
    pub over: bool,
}

/// What a full Legend does to a defender with this Barricade, in order. The
/// defender's numbers change between the Wizard's two bolts, so each hit is
/// worked out against what the one before left. Damage into a Barricade
/// never carries over to health.
pub fn attack(prof: Prof, level: u8, barricade: &mut i32, health: &mut i32) -> Vec<Hit> {
    let (to_health, to_barricade) = prof.damage(level);
    let mut hits = Vec::new();
    let wall_first = |b: &mut i32, h: &mut i32, dmg: i32, hits: &mut Vec<Hit>| {
        if *b > 0 {
            let took = dmg.min(*b);
            *b -= took;
            hits.push(Hit {
                target: Target::Barricade,
                amount: took,
                over: false,
            });
        } else {
            let took = dmg.min(*h);
            *h -= took;
            hits.push(Hit {
                target: Target::Health,
                amount: took,
                over: false,
            });
        }
    };
    match prof {
        Prof::Knight => wall_first(barricade, health, to_health, &mut hits),
        Prof::Wizard => {
            wall_first(barricade, health, to_health, &mut hits);
            let over = *barricade > 0;
            let took = to_health.min(*health);
            *health -= took;
            hits.push(Hit {
                target: Target::Health,
                amount: took,
                over,
            });
        }
        Prof::Bowman => {
            if *barricade >= BOWMAN_HEIGHT {
                let took = to_barricade.min(*barricade);
                *barricade -= took;
                hits.push(Hit {
                    target: Target::Barricade,
                    amount: took,
                    over: false,
                });
            } else {
                let over = *barricade > 0;
                let took = to_health.min(*health);
                *health -= took;
                hits.push(Hit {
                    target: Target::Health,
                    amount: took,
                    over,
                });
            }
        }
    }
    hits
}

#[cfg(test)]
mod tests {
    use super::*;
    use Sym::*;

    fn panels(list: &[(Sym, u8)]) -> Vec<Panel> {
        list.iter().map(|&(s, n)| p(s, n, false)).collect()
    }

    // The four rows of journal tab 2, worked by hand.
    #[test]
    fn three_reds_make_one_segment() {
        let t = tally(&panels(&[
            (Red, 1),
            (Red, 1),
            (Red, 1),
            (Blue, 1),
            (Barricade, 1),
        ]));
        assert_eq!((t.red, t.blue, t.barricade), (3, 1, 1));
        assert_eq!(
            (segments(t.red), segments(t.blue), segments(t.barricade)),
            (1, 0, 0)
        );
    }

    #[test]
    fn a_double_counts_two_symbols() {
        let t = tally(&panels(&[
            (Red, 2),
            (Red, 1),
            (Red, 1),
            (Blue, 2),
            (Barricade, 1),
        ]));
        assert_eq!(segments(t.red), 2);
        assert_eq!(segments(t.blue), 0);
    }

    #[test]
    fn two_kinds_can_pay_out_on_one_turn() {
        let t = tally(&panels(&[
            (Barricade, 2),
            (Barricade, 1),
            (Blue, 1),
            (Blue, 2),
            (Red, 1),
        ]));
        assert_eq!(
            (segments(t.barricade), segments(t.blue), segments(t.red)),
            (1, 1, 0)
        );
    }

    #[test]
    fn ten_reds_make_eight_segments() {
        let t = tally(&panels(&[(Red, 2); 5]));
        assert_eq!(segments(t.red), 8);
    }

    #[test]
    fn gilded_panels_are_counted_once_each_whatever_their_symbols() {
        let t = tally(&[
            p(Red, 2, true),
            p(Red, 1, true),
            p(Blue, 2, true),
            p(Barricade, 1, false),
        ]);
        assert_eq!((t.red_gilded, t.blue_gilded), (2, 1));
    }

    #[test]
    fn every_wheel_carries_the_ten_panels_once() {
        for order in ORDER {
            let mut seen = order;
            seen.sort();
            assert_eq!(seen, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        }
        // the person's list: 4 red, 4 blue, 2 Barricade panels; 4 gilded
        let reds = PINE.iter().filter(|p| p.sym == Red).count();
        let gilded = PINE.iter().filter(|p| p.gilded).count();
        let bars = PINE.iter().filter(|p| p.sym == Barricade).count();
        assert_eq!((reds, gilded, bars), (4, 4, 2));
    }

    #[test]
    fn oak_has_more_doubles_than_pine() {
        let doubles = |g: Grade| g.panels().iter().filter(|p| p.count == 2).count();
        assert_eq!((doubles(Grade::Pine), doubles(Grade::Oak)), (5, 7));
    }

    #[test]
    fn bars_hold_three_five_and_four() {
        assert_eq!(
            (Prof::Knight.cost(), Prof::Wizard.cost(), Prof::Bowman.cost()),
            (3, 5, 4)
        );
    }

    fn hit(prof: Prof, level: u8, b: i32, h: i32) -> (Vec<(Target, i32, bool)>, i32, i32) {
        let (mut b, mut h) = (b, h);
        let hits = attack(prof, level, &mut b, &mut h);
        (
            hits.into_iter().map(|x| (x.target, x.amount, x.over)).collect(),
            b,
            h,
        )
    }

    #[test]
    fn the_knight_hits_any_barricade_and_nothing_carries_over() {
        assert_eq!(
            hit(Prof::Knight, 0, 1, 10),
            (vec![(Target::Barricade, 1, false)], 0, 10)
        );
        assert_eq!(
            hit(Prof::Knight, 0, 5, 10),
            (vec![(Target::Barricade, 3, false)], 2, 10)
        );
        assert_eq!(
            hit(Prof::Knight, 0, 0, 10),
            (vec![(Target::Health, 3, false)], 0, 7)
        );
    }

    #[test]
    fn the_wizards_second_bolt_flies_over_the_barricade() {
        assert_eq!(
            hit(Prof::Wizard, 0, 5, 10),
            (
                vec![(Target::Barricade, 2, false), (Target::Health, 2, true)],
                3,
                8
            )
        );
        assert_eq!(
            hit(Prof::Wizard, 0, 0, 10),
            (vec![(Target::Health, 2, false), (Target::Health, 2, false)], 0, 6)
        );
        // the first bolt breaks a Barricade of 1; the second meets none
        assert_eq!(
            hit(Prof::Wizard, 0, 1, 10),
            (
                vec![(Target::Barricade, 1, false), (Target::Health, 2, false)],
                0,
                8
            )
        );
    }

    #[test]
    fn the_bowman_passes_a_barricade_of_two_and_chips_one_of_three() {
        assert_eq!(
            hit(Prof::Bowman, 0, 2, 10),
            (vec![(Target::Health, 3, true)], 2, 7)
        );
        assert_eq!(
            hit(Prof::Bowman, 0, 0, 10),
            (vec![(Target::Health, 3, false)], 0, 7)
        );
        assert_eq!(
            hit(Prof::Bowman, 0, 3, 10),
            (vec![(Target::Barricade, 1, false)], 2, 10)
        );
    }

    #[test]
    fn each_level_adds_one_to_every_damage_number() {
        assert_eq!(hit(Prof::Knight, 2, 0, 10).2, 5);
        assert_eq!(hit(Prof::Bowman, 1, 5, 10).1, 3);
        assert_eq!(hit(Prof::Wizard, 1, 0, 10).2, 4);
    }

    #[test]
    fn health_never_goes_below_zero() {
        assert_eq!(
            hit(Prof::Knight, 0, 0, 2),
            (vec![(Target::Health, 2, false)], 0, 0)
        );
    }
}
