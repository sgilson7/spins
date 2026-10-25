//! The door into the core. A `Session` holds the whole game, including the
//! match, inside the WebAssembly module, so the opponent's faces never reach
//! the page before the reveal (SP-4). The page calls `act(name, value)` and
//! draws the `Ui` that comes back: the screen's panels, the table during a
//! match, and the events of the turn that just ended. Every view (2D, 3D,
//! pixel) draws the same `Ui` (SP-1). (After Peter's Mandible.)

use std::collections::BTreeMap;

use serde::Serialize;

use crate::ai::Skill;
use crate::glossary;
use crate::rng::Rng;
use crate::rules::{self, Grade, Prof, Sym, Target};
use crate::save::Progress;
use crate::table::{Event, Match, Outcome, Setup, Side};

pub struct Opponent {
    pub key: &'static str,
    pub skill: Skill,
    pub legends: [Prof; 2],
    pub grade: Grade,
}

/// The tavern's three opponents, easy to hard (SP-Q15). The last is the
/// champion: beating them wins oak wheels (T4, from SP-Q18).
pub const OPPONENTS: [Opponent; 3] = [
    Opponent {
        key: "wenna",
        skill: Skill::Easy,
        legends: [Prof::Knight, Prof::Wizard],
        grade: Grade::Pine,
    },
    Opponent {
        key: "tobble",
        skill: Skill::Fair,
        legends: [Prof::Wizard, Prof::Bowman],
        grade: Grade::Pine,
    },
    Opponent {
        key: "halvard",
        skill: Skill::Hard,
        legends: [Prof::Bowman, Prof::Knight],
        grade: Grade::Oak,
    },
];
pub const CHAMPION: usize = 2;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Action {
    pub name: String,
    pub value: String,
    pub label: String,
    pub enabled: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub style: String,
}

/// One block of a screen. `kind` names the block's look (sign, card,
/// notice, scroll); `art` names a picture the view draws (a proficiency, a
/// grade); `mark` is a seat (red, blue). Empty fields are left out.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct Panel {
    pub id: String,
    pub kind: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub heading: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub text: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lines: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub art: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub mark: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Hint {
    pub term: String,
    pub definition: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PanelView {
    pub sym: Sym,
    pub count: u8,
    pub gilded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WheelView {
    pub face: usize,
    pub locked: bool,
    /// the wheel's panels in order around it, so a view can draw the drum
    pub panels: Vec<PanelView>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LegendView {
    pub seat: String,
    pub prof: Prof,
    pub name: String,
    pub energy: i32,
    pub cost: i32,
    pub xp: i32,
    pub xp_max: i32,
    pub level: u8,
    pub level_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SideView {
    pub name: String,
    pub health: i32,
    pub health_max: i32,
    pub barricade: i32,
    pub barricade_max: i32,
    pub legends: [LegendView; 2],
    /// empty while the side is covered
    pub wheels: Vec<WheelView>,
    pub covered: bool,
    pub spins_used: u8,
    pub spins_max: u8,
    pub grade: Grade,
}

/// Symbols of each kind showing on your wheels and the segments they would
/// make if the turn ended now.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Count {
    pub sym: Sym,
    pub symbols: i32,
    pub segments: i32,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TableView {
    pub turn: u32,
    pub sides: [SideView; 2],
    pub counts: Vec<Count>,
    pub prompt: String,
    pub actions: Vec<Action>,
    /// one lock action per wheel
    pub locks: Vec<Action>,
    pub words: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EventView {
    #[serde(flatten)]
    pub event: Event,
    pub text: String,
    /// for a reveal: both sides' wheels, as the table shows them
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wheels: Option<[Vec<WheelView>; 2]>,
    /// for an evolve: the new level's name
    #[serde(skip_serializing_if = "String::is_empty")]
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Ui {
    pub screen: String,
    pub title: String,
    pub panels: Vec<Panel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<TableView>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<EventView>,
    /// text for the page to store, when progress changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save: Option<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub notice: String,
    pub hints: Vec<Hint>,
}

fn ok<T: Serialize>(value: &T) -> String {
    serde_json::json!({ "ok": value }).to_string()
}

fn error(message: String) -> String {
    serde_json::json!({ "error": message }).to_string()
}

/// The reader-visible strings, flat, keyed by dotted name.
pub struct Strings(BTreeMap<String, String>);

impl Strings {
    pub fn parse(json: &str) -> Result<Strings, String> {
        let v: serde_json::Value =
            serde_json::from_str(json).map_err(|e| format!("The strings file could not be read: {e}"))?;
        let mut out = BTreeMap::new();
        flatten(&v, "", &mut out);
        for key in REQUIRED {
            if out
                .get(*key)
                .map(|s: &String| s.trim().is_empty())
                .unwrap_or(true)
            {
                return Err(format!("The strings file has no \"{key}\"."));
            }
        }
        Ok(Strings(out))
    }

    pub fn get(&self, key: &str) -> String {
        self.0.get(key).cloned().unwrap_or_else(|| format!("[{key}]"))
    }
}

/// Keys the start screen cannot do without; a test plays every screen and
/// checks that no other key is missing.
const REQUIRED: &[&str] = &["title", "title.tagline"];

fn flatten(v: &serde_json::Value, at: &str, out: &mut BTreeMap<String, String>) {
    match v {
        serde_json::Value::String(s) => {
            out.insert(at.to_string(), s.clone());
        }
        serde_json::Value::Object(map) => {
            for (k, x) in map {
                if k.starts_with('_') {
                    continue;
                }
                // "x": {"": "..."} names the key itself
                let key = match (at.is_empty(), k.is_empty()) {
                    (true, _) => k.clone(),
                    (false, true) => at.to_string(),
                    (false, false) => format!("{at}.{k}"),
                };
                flatten(x, &key, out);
            }
        }
        _ => {}
    }
}

/// Fills `{name}` placeholders.
pub fn fill(template: &str, values: &[(&str, String)]) -> String {
    let mut s = template.to_string();
    for (k, v) in values {
        s = s.replace(&format!("{{{k}}}"), v);
    }
    s
}

/// The numbers the glossary and the rules scroll name, filled from the rules.
fn rule_values() -> Vec<(&'static str, String)> {
    vec![
        ("health", rules::HEALTH.to_string()),
        ("barricade.max", rules::BARRICADE_MAX.to_string()),
        ("spins", rules::SPINS.to_string()),
        ("wheels", rules::WHEELS.to_string()),
        ("panels", rules::PANELS.to_string()),
        ("oak", rules::OAK.to_string()),
        ("xp.max", rules::XP_MAX.to_string()),
        ("xp.action", rules::XP_PER_ACTION.to_string()),
        ("burst", rules::GOLD_BURST.to_string()),
        ("bowman.height", rules::BOWMAN_HEIGHT.to_string()),
        ("knight.cost", Prof::Knight.cost().to_string()),
        ("wizard.cost", Prof::Wizard.cost().to_string()),
        ("bowman.cost", Prof::Bowman.cost().to_string()),
        ("knight.damage", Prof::Knight.damage(0).0.to_string()),
        ("wizard.damage", Prof::Wizard.damage(0).0.to_string()),
        ("bowman.damage", Prof::Bowman.damage(0).0.to_string()),
        ("bowman.chip", Prof::Bowman.damage(0).1.to_string()),
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Title,
    Rules,
    Legends,
    Opponents,
    Match,
    Over,
}

impl Screen {
    fn key(self) -> &'static str {
        match self {
            Screen::Title => "title",
            Screen::Rules => "rules",
            Screen::Legends => "legends",
            Screen::Opponents => "opponents",
            Screen::Match => "match",
            Screen::Over => "over",
        }
    }
}

pub struct Session {
    s: Strings,
    hints: Vec<Hint>,
    screen: Screen,
    /// the proficiencies chosen, Red first
    pick: Vec<Prof>,
    grade: Grade,
    opponent: usize,
    m: Option<Match>,
    events: Vec<EventView>,
    progress: Progress,
    save: Option<String>,
    notice: String,
    rng: Rng,
}

impl Session {
    pub fn new(strings_json: &str, glossary_md: &str, save_text: &str, seed: u32) -> Result<Session, String> {
        let s = Strings::parse(strings_json)?;
        let values = rule_values();
        let hints = glossary::parse(glossary_md)
            .map_err(|e| e.message())?
            .into_iter()
            .map(|e| Hint {
                term: e.term,
                definition: fill(&e.definition, &values),
            })
            .collect();
        let (progress, refused) = Progress::load(save_text, OPPONENTS.len());
        let notice = if refused {
            s.get("notice.save_refused")
        } else {
            String::new()
        };
        Ok(Session {
            s,
            hints,
            screen: Screen::Title,
            pick: vec![Prof::Knight, Prof::Wizard],
            grade: Grade::Pine,
            opponent: 0,
            m: None,
            events: Vec::new(),
            progress,
            save: None,
            notice,
            rng: Rng::new(seed as u64),
        })
    }

    /// Applies one named action and returns the next screen, or an error the
    /// page shows as it is.
    pub fn act(&mut self, name: &str, value: &str) -> String {
        self.events.clear();
        self.save = None;
        self.notice.clear();
        match self.apply(name, value) {
            Ok(()) => ok(&self.ui()),
            Err(e) => error(e),
        }
    }

    pub fn ui_json(&self) -> String {
        ok(&self.ui())
    }

    fn apply(&mut self, name: &str, value: &str) -> Result<(), String> {
        match name {
            "screen.show" => {
                self.screen = match value {
                    "title" => Screen::Title,
                    "rules" => Screen::Rules,
                    "legends" => Screen::Legends,
                    "opponents" if self.pick.len() == 2 => Screen::Opponents,
                    other => return Err(format!("There is no screen \"{other}\".")),
                };
                if self.screen != Screen::Match {
                    self.m = None;
                }
            }
            "legend.pick" => {
                let p = Prof::from_key(value).ok_or("There is no such proficiency.")?;
                if let Some(i) = self.pick.iter().position(|&x| x == p) {
                    self.pick.remove(i);
                } else if self.pick.len() < 2 {
                    self.pick.push(p);
                } else {
                    self.pick[1] = p;
                }
            }
            "legend.swap" => self.pick.reverse(),
            "wheels.grade" => {
                self.grade = match value {
                    "oak" if self.progress.oak => Grade::Oak,
                    "pine" => Grade::Pine,
                    _ => return Err(self.s.get("notice.oak_locked")),
                }
            }
            "opponent.pick" => {
                let i = OPPONENTS
                    .iter()
                    .position(|o| o.key == value)
                    .ok_or("There is no such opponent.")?;
                self.opponent = i;
                self.start_match()?;
            }
            "match.spin" => {
                let m = self.m.as_mut().ok_or("No match is being played.")?;
                let ev = m.spin()?;
                self.after_turn(ev);
            }
            "match.end" => {
                let m = self.m.as_mut().ok_or("No match is being played.")?;
                let ev = m.end_turn()?;
                self.after_turn(ev);
            }
            "match.lock" => {
                let w: usize = value.parse().map_err(|_| "There is no such wheel.")?;
                self.m
                    .as_mut()
                    .ok_or("No match is being played.")?
                    .toggle_lock(w)?;
            }
            "match.leave" => {
                self.m = None;
                self.screen = Screen::Title;
            }
            "over.rematch" => self.start_match()?,
            "progress.forget" => {
                self.progress = Progress::fresh(OPPONENTS.len());
                self.grade = Grade::Pine;
                self.save = Some(String::new());
            }
            other => return Err(format!("Unknown action \"{other}\".")),
        }
        Ok(())
    }

    fn start_match(&mut self) -> Result<(), String> {
        if self.pick.len() != 2 {
            return Err(self.s.get("notice.pick_two"));
        }
        let o = &OPPONENTS[self.opponent];
        let setup = Setup {
            you: [self.pick[0], self.pick[1]],
            you_grade: self.grade,
            them: o.legends,
            them_grade: o.grade,
            skill: o.skill,
        };
        self.m = Some(Match::new(&setup, self.rng.next_u64()));
        self.screen = Screen::Match;
        Ok(())
    }

    fn after_turn(&mut self, ev: Vec<Event>) {
        let views: Vec<EventView> = ev.into_iter().map(|e| self.narrate(e)).collect();
        self.events = views;
        let Some(outcome) = self.m.as_ref().and_then(|m| m.outcome) else {
            return;
        };
        let r = &mut self.progress.records[self.opponent];
        match outcome {
            Outcome::Win => r.wins += 1,
            Outcome::Loss => r.losses += 1,
            Outcome::Draw => r.draws += 1,
        }
        if outcome == Outcome::Win && self.opponent == CHAMPION && !self.progress.oak {
            self.progress.oak = true;
            self.notice = self.s.get("notice.oak_won");
        }
        self.save = Some(self.progress.text());
        self.screen = Screen::Over;
    }

    fn opponent_name(&self) -> String {
        self.s
            .get(&format!("opponent.{}.name", OPPONENTS[self.opponent].key))
    }

    fn who(&self, side: usize) -> (String, String) {
        if side == 0 {
            (self.s.get("you.who"), self.s.get("you.whose"))
        } else {
            let n = self.opponent_name();
            (n.clone(), fill(&self.s.get("them.whose"), &[("name", n)]))
        }
    }

    /// A side named as the one hit, in the middle of a sentence: "you" and
    /// "your" for the person, the opponent's name otherwise.
    fn foe_names(&self, side: usize) -> (String, String) {
        if side == 0 {
            (self.s.get("you.foe"), self.s.get("you.foes"))
        } else {
            self.who(side)
        }
    }

    fn legend_name(&self, side: usize, slot: usize) -> String {
        let m = self
            .m
            .as_ref()
            .map(|m| m.sides[side].legends[slot].prof)
            .unwrap_or(Prof::Knight);
        self.s.get(&format!("prof.{}.name", m.key()))
    }

    fn sym_name(&self, sym: Sym) -> String {
        self.s.get(match sym {
            Sym::Red => "sym.red",
            Sym::Blue => "sym.blue",
            Sym::Barricade => "sym.barricade",
        })
    }

    fn narrate(&self, event: Event) -> EventView {
        let s = &self.s;
        let text = match &event {
            Event::Reveal { .. } => s.get("event.reveal"),
            Event::Count {
                side,
                sym,
                symbols,
                segments,
            } => fill(
                &s.get("event.count"),
                &[
                    ("who", self.who(*side).0),
                    ("symbols", symbols.to_string()),
                    ("sym", self.sym_name(*sym)),
                    ("segments", segments.to_string()),
                ],
            ),
            Event::Barricade { side, value, .. } => fill(
                &s.get("event.barricade"),
                &[("whose", self.who(*side).1), ("value", value.to_string())],
            ),
            Event::Energy {
                side, slot, value, ..
            } => {
                let m = self
                    .m
                    .as_ref()
                    .map(|m| m.sides[*side].legends[*slot].prof.cost())
                    .unwrap_or(0);
                fill(
                    &s.get("event.energy"),
                    &[
                        ("whose", self.who(*side).1),
                        ("legend", self.legend_name(*side, *slot)),
                        ("value", value.to_string()),
                        ("cost", m.to_string()),
                    ],
                )
            }
            Event::Xp {
                side,
                slot,
                value,
                added,
            } => fill(
                &s.get("event.xp"),
                &[
                    ("whose", self.who(*side).1),
                    ("legend", self.legend_name(*side, *slot)),
                    ("added", added.to_string()),
                    ("value", value.to_string()),
                    ("max", rules::XP_MAX.to_string()),
                ],
            ),
            Event::Evolve { side, slot, level } => fill(
                &s.get("event.evolve"),
                &[
                    ("whose", self.who(*side).1),
                    ("legend", self.legend_name(*side, *slot)),
                    ("level", s.get(&format!("level.{level}"))),
                ],
            ),
            Event::Burst {
                side, slot, amount, ..
            } => fill(
                &s.get("event.burst"),
                &[
                    ("whose", self.who(*side).1),
                    ("legend", self.legend_name(*side, *slot)),
                    ("foe", self.foe_names(1 - side).0),
                    ("amount", amount.to_string()),
                ],
            ),
            Event::Act { side, slot, .. } => fill(
                &s.get("event.act"),
                &[
                    ("whose", self.who(*side).1),
                    ("legend", self.legend_name(*side, *slot)),
                ],
            ),
            Event::Hit {
                side,
                target,
                amount,
                over,
                ..
            } => {
                let key = match (target, over) {
                    (Target::Barricade, _) => "event.hit.barricade",
                    (Target::Health, true) => "event.hit.over",
                    (Target::Health, false) => "event.hit.health",
                };
                let (foe, foes) = self.foe_names(1 - side);
                fill(
                    &s.get(key),
                    &[("foe", foe), ("foes", foes), ("amount", amount.to_string())],
                )
            }
            Event::Over { outcome } => s.get(&format!("event.over.{}", outcome_key(*outcome))),
        };
        let wheels = match (&event, self.m.as_ref()) {
            (Event::Reveal { .. }, Some(m)) => Some([wheel_views(&m.sides[0]), wheel_views(&m.sides[1])]),
            _ => None,
        };
        let label = match &event {
            Event::Evolve { level, .. } => s.get(&format!("level.{level}")),
            _ => String::new(),
        };
        EventView {
            event,
            text,
            wheels,
            label,
        }
    }

    fn action(&self, name: &str, value: &str, label_key: &str, enabled: bool, style: &str) -> Action {
        Action {
            name: name.into(),
            value: value.into(),
            label: self.s.get(label_key),
            enabled,
            style: style.into(),
        }
    }

    pub fn ui(&self) -> Ui {
        let panels = match self.screen {
            Screen::Title => self.title_panels(),
            Screen::Rules => self.rules_panels(),
            Screen::Legends => self.legend_panels(),
            Screen::Opponents => self.opponent_panels(),
            Screen::Match => Vec::new(),
            Screen::Over => self.over_panels(),
        };
        let table = match self.screen {
            Screen::Match | Screen::Over => self.m.as_ref().map(|m| self.table(m)),
            _ => None,
        };
        Ui {
            screen: self.screen.key().into(),
            title: self.s.get("title"),
            panels,
            table,
            events: self.events.clone(),
            save: self.save.clone(),
            notice: self.notice.clone(),
            hints: self.hints.clone(),
        }
    }

    fn title_panels(&self) -> Vec<Panel> {
        let mut actions = vec![
            self.action("screen.show", "legends", "action.play", true, "primary"),
            self.action("screen.show", "rules", "action.rules", true, ""),
        ];
        let played: u32 = self
            .progress
            .records
            .iter()
            .map(|r| r.wins + r.losses + r.draws)
            .sum();
        let mut lines = Vec::new();
        if played > 0 {
            let wins: u32 = self.progress.records.iter().map(|r| r.wins).sum();
            lines.push(fill(
                &self.s.get("title.record"),
                &[("wins", wins.to_string()), ("played", played.to_string())],
            ));
            actions.push(self.action("progress.forget", "", "action.forget", true, "quiet"));
        }
        vec![Panel {
            id: "sign".into(),
            kind: "sign".into(),
            heading: self.s.get("title"),
            text: self.s.get("title.tagline"),
            lines,
            actions,
            ..Panel::default()
        }]
    }

    fn rules_panels(&self) -> Vec<Panel> {
        let values = rule_values();
        let lines = (1..=9)
            .map(|i| fill(&self.s.get(&format!("rules.{i}")), &values))
            .collect();
        vec![Panel {
            id: "rules".into(),
            kind: "scroll".into(),
            heading: self.s.get("rules.heading"),
            lines,
            actions: vec![self.action("screen.show", "title", "action.back", true, "")],
            ..Panel::default()
        }]
    }

    fn prof_lines(&self, p: Prof) -> Vec<String> {
        let values = rule_values();
        vec![
            fill(&self.s.get("prof.bar"), &[("cost", p.cost().to_string())]),
            fill(&self.s.get(&format!("prof.{}.does", p.key())), &values),
        ]
    }

    fn legend_panels(&self) -> Vec<Panel> {
        let mut out: Vec<Panel> = Prof::ALL
            .iter()
            .map(|&p| {
                let seat = self.pick.iter().position(|&x| x == p);
                let mark = match seat {
                    Some(0) => "red",
                    Some(_) => "blue",
                    None => "",
                };
                let label = if seat.is_some() {
                    "action.unpick"
                } else {
                    "action.pick"
                };
                Panel {
                    id: format!("card-{}", p.key()),
                    kind: "card".into(),
                    heading: self.s.get(&format!("prof.{}.name", p.key())),
                    text: match seat {
                        Some(0) => self.s.get("seat.red"),
                        Some(_) => self.s.get("seat.blue"),
                        None => String::new(),
                    },
                    lines: self.prof_lines(p),
                    art: p.key().into(),
                    mark: mark.into(),
                    actions: vec![self.action("legend.pick", p.key(), label, true, "")],
                }
            })
            .collect();
        let mut actions = vec![
            self.action("legend.swap", "", "action.swap", self.pick.len() == 2, ""),
            self.action(
                "screen.show",
                "opponents",
                "action.next",
                self.pick.len() == 2,
                "primary",
            ),
            self.action("screen.show", "title", "action.back", true, "quiet"),
        ];
        let mut lines = vec![self.s.get("legends.help")];
        if self.progress.oak {
            lines.push(self.s.get(&format!("grade.{}.line", self.grade.key())));
            actions.insert(
                0,
                self.action(
                    "wheels.grade",
                    "pine",
                    "grade.pine.name",
                    self.grade != Grade::Pine,
                    "",
                ),
            );
            actions.insert(
                1,
                self.action(
                    "wheels.grade",
                    "oak",
                    "grade.oak.name",
                    self.grade != Grade::Oak,
                    "",
                ),
            );
        }
        out.push(Panel {
            id: "seat".into(),
            kind: "bar".into(),
            heading: self.s.get("legends.heading"),
            lines,
            actions,
            ..Panel::default()
        });
        out
    }

    fn opponent_panels(&self) -> Vec<Panel> {
        let mut out = vec![Panel {
            id: "tavern".into(),
            kind: "bar".into(),
            heading: self.s.get("tavern.name"),
            text: self.s.get("tavern.help"),
            actions: vec![self.action("screen.show", "legends", "action.back", true, "quiet")],
            ..Panel::default()
        }];
        for (i, o) in OPPONENTS.iter().enumerate() {
            let r = &self.progress.records[i];
            let names = o.legends.map(|p| self.s.get(&format!("prof.{}.name", p.key())));
            let mut lines = vec![
                self.s.get(&format!("skill.{}", skill_key(o.skill))),
                fill(
                    &self.s.get("opponent.legends"),
                    &[("red", names[0].clone()), ("blue", names[1].clone())],
                ),
                self.s.get(&format!("grade.{}.line", o.grade.key())),
            ];
            if r.wins + r.losses + r.draws > 0 {
                lines.push(fill(
                    &self.s.get("opponent.record"),
                    &[("wins", r.wins.to_string()), ("losses", r.losses.to_string())],
                ));
            }
            out.push(Panel {
                id: format!("opponent-{}", o.key),
                kind: "notice".into(),
                heading: self.s.get(&format!("opponent.{}.name", o.key)),
                text: self.s.get(&format!("opponent.{}.about", o.key)),
                lines,
                art: if i == CHAMPION {
                    "champion".into()
                } else {
                    String::new()
                },
                actions: vec![self.action("opponent.pick", o.key, "action.sit", true, "primary")],
                ..Panel::default()
            });
        }
        out
    }

    fn over_panels(&self) -> Vec<Panel> {
        let Some(m) = self.m.as_ref() else {
            return Vec::new();
        };
        let outcome = m.outcome.unwrap_or(Outcome::Draw);
        let k = outcome_key(outcome);
        let name = self.opponent_name();
        let mut lines = vec![fill(
            &self.s.get("over.health"),
            &[
                ("you", m.sides[0].health.to_string()),
                ("them", m.sides[1].health.to_string()),
                ("name", name.clone()),
            ],
        )];
        for slot in 0..2 {
            lines.push(fill(
                &self.s.get("over.acts"),
                &[
                    ("legend", self.legend_name(0, slot)),
                    ("n", m.stats.acts[0][slot].to_string()),
                ],
            ));
        }
        if !self.notice.is_empty() {
            lines.push(self.notice.clone());
        }
        vec![Panel {
            id: "over".into(),
            kind: "scroll".into(),
            heading: self.s.get(&format!("over.{k}")),
            text: fill(
                &self.s.get("over.turn"),
                &[("turn", m.turn.to_string()), ("name", name)],
            ),
            lines,
            mark: k.into(),
            actions: vec![
                self.action("over.rematch", "", "action.rematch", true, "primary"),
                self.action("screen.show", "opponents", "action.tavern", true, ""),
                self.action("screen.show", "title", "action.title", true, "quiet"),
            ],
            ..Panel::default()
        }]
    }

    fn side_view(&self, m: &Match, i: usize) -> SideView {
        let side: &Side = &m.sides[i];
        let covered = i == 1 && m.outcome.is_none();
        let wheels = if covered { Vec::new() } else { wheel_views(side) };
        let legend = |slot: usize| {
            let l = &side.legends[slot];
            LegendView {
                seat: if slot == 0 { "red".into() } else { "blue".into() },
                prof: l.prof,
                name: self.s.get(&format!("prof.{}.name", l.prof.key())),
                energy: l.energy,
                cost: l.prof.cost(),
                xp: l.xp,
                xp_max: rules::XP_MAX,
                level: l.level,
                level_name: self.s.get(&format!("level.{}", l.level)),
            }
        };
        SideView {
            name: if i == 0 {
                self.s.get("you.name")
            } else {
                self.opponent_name()
            },
            health: side.health,
            health_max: rules::HEALTH,
            barricade: side.barricade,
            barricade_max: rules::BARRICADE_MAX,
            legends: [legend(0), legend(1)],
            wheels,
            covered,
            spins_used: side.spins_used,
            spins_max: rules::SPINS,
            grade: side.grade,
        }
    }

    fn table(&self, m: &Match) -> TableView {
        let t = m.sides[0].tally();
        let counts = [
            (Sym::Red, t.red),
            (Sym::Blue, t.blue),
            (Sym::Barricade, t.barricade),
        ]
        .into_iter()
        .map(|(sym, n)| Count {
            sym,
            symbols: n,
            segments: rules::segments(n),
            text: fill(
                &self.s.get("tally.count"),
                &[
                    ("symbols", n.to_string()),
                    ("segments", rules::segments(n).to_string()),
                ],
            ),
        })
        .collect();
        let you = &m.sides[0];
        let left = (rules::SPINS - you.spins_used).to_string();
        let prompt = if m.outcome.is_some() {
            String::new()
        } else if you.spins_used == 0 {
            fill(&self.s.get("prompt.start"), &[("turn", m.turn.to_string())])
        } else if you.all_locked() {
            self.s.get("prompt.all_locked")
        } else {
            fill(&self.s.get("prompt.lock"), &[("left", left.clone())])
        };
        let spin_label = fill(
            &self.s.get("action.spin"),
            &[
                ("n", (you.spins_used + 1).min(rules::SPINS).to_string()),
                ("of", rules::SPINS.to_string()),
            ],
        );
        let actions = vec![
            Action {
                name: "match.spin".into(),
                value: String::new(),
                label: spin_label,
                enabled: m.can_spin(),
                style: "primary".into(),
            },
            self.action("match.end", "", "action.end", m.can_end(), ""),
            self.action("match.leave", "", "action.leave", true, "quiet"),
        ];
        let locks = (0..rules::WHEELS)
            .map(|w| Action {
                name: "match.lock".into(),
                value: w.to_string(),
                label: fill(
                    &self.s.get(if you.locks[w] {
                        "action.unlock"
                    } else {
                        "action.lock"
                    }),
                    &[("n", (w + 1).to_string())],
                ),
                enabled: m.can_lock(),
                style: if you.locks[w] { "on".into() } else { String::new() },
            })
            .collect();
        let words = self
            .s
            .0
            .iter()
            .filter_map(|(k, v)| k.strip_prefix("table.").map(|k| (k.to_string(), v.clone())))
            .collect();
        TableView {
            turn: m.turn,
            sides: [self.side_view(m, 0), self.side_view(m, 1)],
            counts,
            prompt,
            actions,
            locks,
            words,
        }
    }
}

fn wheel_views(side: &Side) -> Vec<WheelView> {
    (0..rules::WHEELS)
        .map(|w| WheelView {
            face: side.faces[w],
            locked: side.locks[w],
            panels: (0..rules::PANELS)
                .map(|f| {
                    let p = side.grade.panel(w, f);
                    PanelView {
                        sym: p.sym,
                        count: p.count,
                        gilded: p.gilded,
                    }
                })
                .collect(),
        })
        .collect()
}

fn outcome_key(o: Outcome) -> &'static str {
    match o {
        Outcome::Win => "win",
        Outcome::Loss => "loss",
        Outcome::Draw => "draw",
    }
}

fn skill_key(s: Skill) -> &'static str {
    match s {
        Skill::Easy => "easy",
        Skill::Fair => "fair",
        Skill::Hard => "hard",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STRINGS: &str = include_str!("../../../data/strings.json");
    const GLOSSARY: &str = include_str!("../../../GLOSSARY.md");

    fn json(text: &str) -> serde_json::Value {
        serde_json::from_str(text).unwrap()
    }

    fn session() -> Session {
        Session::new(STRINGS, GLOSSARY, "", 7).unwrap()
    }

    fn go(s: &mut Session, name: &str, value: &str) -> serde_json::Value {
        let v = json(&s.act(name, value));
        assert!(v.get("ok").is_some(), "{name} {value}: {v}");
        v["ok"].clone()
    }

    /// Plays to the end with a fixed habit: spin, lock wheel 1, spin, spin.
    fn play_to_the_end(s: &mut Session) -> Vec<serde_json::Value> {
        let mut seen = Vec::new();
        for _ in 0..200 {
            let u = go(s, "match.spin", "");
            seen.push(u.clone());
            if u["screen"] == "over" {
                break;
            }
            if u["table"]["sides"][0]["spins_used"] == 1 {
                go(s, "match.lock", "0");
            }
        }
        seen
    }

    /// `"[some.key]"`: what `Strings::get` gives for a key the file lacks.
    fn has_missing_key(t: &str) -> bool {
        t.split("\"[").skip(1).any(|rest| {
            let key: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '.' || *c == '_')
                .collect();
            !key.is_empty() && rest[key.len()..].starts_with("]\"")
        })
    }

    /// `{name}` left in a string; JSON's own braces are followed by a quote.
    fn has_placeholder(t: &str) -> bool {
        t.split('{')
            .skip(1)
            .any(|rest| rest.starts_with(|c: char| c.is_ascii_lowercase()))
    }

    #[test]
    fn the_missing_string_checks_can_fail() {
        assert!(has_missing_key(r#"{"a":"[opponent.wenna.name]"}"#));
        assert!(!has_missing_key(r#"{"a":["x"]}"#));
        assert!(has_placeholder(r#"{"a":"hits {foe}"}"#));
        assert!(!has_placeholder(r#"{"a":{"b":1}}"#));
    }

    #[test]
    fn the_game_opens_on_the_title_with_its_hints_filled() {
        let u = json(&session().ui_json());
        assert_eq!(u["ok"]["screen"], "title");
        let hints = u["ok"]["hints"].as_array().unwrap();
        assert!(!hints.is_empty());
        assert!(hints
            .iter()
            .all(|h| !h["definition"].as_str().unwrap().contains('{')));
    }

    #[test]
    fn the_opponents_faces_never_reach_the_page_before_the_reveal() {
        let mut s = session();
        go(&mut s, "screen.show", "legends");
        go(&mut s, "screen.show", "opponents");
        let u = go(&mut s, "opponent.pick", "halvard");
        assert_eq!(u["table"]["sides"][1]["covered"], true);
        assert_eq!(u["table"]["sides"][1]["wheels"].as_array().unwrap().len(), 0);
        let u = go(&mut s, "match.spin", "");
        assert_eq!(u["table"]["sides"][1]["wheels"].as_array().unwrap().len(), 0);
        assert!(u["events"].is_null(), "no events before the turn ends");
        assert_eq!(u["table"]["sides"][0]["wheels"].as_array().unwrap().len(), 5);
    }

    #[test]
    fn every_screen_and_event_finds_its_strings() {
        let mut s = session();
        let mut texts = vec![s.ui_json()];
        for (n, v) in [
            ("screen.show", "rules"),
            ("screen.show", "title"),
            ("screen.show", "legends"),
        ] {
            texts.push(s.act(n, v));
        }
        texts.push(s.act("legend.pick", "bowman"));
        texts.push(s.act("legend.swap", ""));
        texts.push(s.act("screen.show", "opponents"));
        for key in ["wenna", "tobble", "halvard"] {
            texts.push(s.act("opponent.pick", key));
            for u in play_to_the_end(&mut s) {
                texts.push(u.to_string());
            }
            texts.push(s.act("screen.show", "opponents"));
        }
        texts.push(s.act("screen.show", "title"));
        for t in texts {
            assert!(!has_missing_key(&t), "a string is missing: {t}");
            assert!(!has_placeholder(&t), "an unfilled placeholder: {t}");
        }
    }

    #[test]
    fn a_finished_match_saves_its_result_and_the_save_loads_back() {
        let mut s = session();
        go(&mut s, "screen.show", "legends");
        go(&mut s, "screen.show", "opponents");
        go(&mut s, "opponent.pick", "wenna");
        let last = play_to_the_end(&mut s).pop().unwrap();
        let text = last["save"].as_str().unwrap().to_string();
        let back = Session::new(STRINGS, GLOSSARY, &text, 1).unwrap();
        let r = &back.progress.records[0];
        assert_eq!(r.wins + r.losses + r.draws, 1);
    }

    #[test]
    fn oak_wheels_wait_until_the_champion_is_beaten() {
        let mut s = session();
        assert!(json(&s.act("wheels.grade", "oak")).get("error").is_some());
        s.progress.oak = true;
        go(&mut s, "wheels.grade", "oak");
        assert_eq!(s.grade, Grade::Oak);
    }

    #[test]
    fn picking_a_third_legend_replaces_the_blue_one() {
        let mut s = session();
        go(&mut s, "legend.pick", "bowman");
        assert_eq!(s.pick, vec![Prof::Knight, Prof::Bowman]);
        go(&mut s, "legend.pick", "knight");
        assert_eq!(s.pick, vec![Prof::Bowman]);
        assert!(json(&s.act("screen.show", "opponents")).get("error").is_some());
    }

    #[test]
    fn an_unknown_action_is_refused_by_name() {
        let v = json(&session().act("nope", ""));
        assert_eq!(v["error"], "Unknown action \"nope\".");
    }
}
