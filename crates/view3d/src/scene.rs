//! The tavern table in 3D. Everything is placed from the table the core
//! described; the scene's own state is only animation: drum angles easing to
//! their faces, the opponent's cover lifting, shots in flight.

use serde::Deserialize;

use crate::fx::{self, int, ratio, ONE, TURN};
use crate::geom::{v3, Affine, V3};
use crate::mesh::Mesh;
use crate::render::{self, Batch, Camera, Rgb, LAYER_INKED, LAYER_PLAIN};

pub const STYLE_GLOW: i32 = 4;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PanelIn {
    pub sym: String,
    pub count: u8,
    pub gilded: bool,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct WheelIn {
    pub face: usize,
    pub locked: bool,
    pub panels: Vec<PanelIn>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LegendIn {
    pub seat: String,
    pub prof: String,
    pub energy: i32,
    pub cost: i32,
    pub xp: i32,
    pub xp_max: i32,
    pub level: u8,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SideIn {
    pub health: i32,
    pub health_max: i32,
    pub barricade: i32,
    pub barricade_max: i32,
    pub legends: Vec<LegendIn>,
    pub wheels: Vec<WheelIn>,
    pub covered: bool,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TableIn {
    pub sides: Vec<SideIn>,
}

/// The parts of an event the scene animates.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct EventIn {
    pub kind: String,
    #[serde(default)]
    pub side: usize,
    #[serde(default)]
    pub slot: usize,
    #[serde(default)]
    pub foe: usize,
    #[serde(default)]
    pub target: String,
    #[serde(default)]
    pub over: bool,
    #[serde(default)]
    pub prof: String,
}

#[derive(Debug, Clone, Copy, Default)]
struct Ease {
    from: i32,
    to: i32,
    start: i64,
    dur: i64,
}

impl Ease {
    fn at(&self, now: i64) -> i32 {
        if self.dur <= 0 || now >= self.start + self.dur {
            return self.to;
        }
        let t = (((now - self.start).max(0) * ONE as i64) / self.dur) as i32;
        // ease out: 1 - (1 - t)^3
        let u = ONE - t;
        let e = ONE - fx::mul(fx::mul(u, u), u);
        self.from + fx::mul(self.to - self.from, e)
    }
    fn moving(&self, now: i64) -> bool {
        now < self.start + self.dur
    }
}

#[derive(Debug, Clone)]
struct Shot {
    from: V3,
    to: V3,
    peak: i32,
    start: i64,
    dur: i64,
    look: ShotLook,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShotLook {
    Blade(Rgb),
    Bolt(Rgb),
    Arrow,
    Burst,
}

#[derive(Debug, Clone, Copy)]
struct Pulse {
    side: usize,
    slot: usize,
    start: i64,
    dur: i64,
    glow: bool,
}

pub struct Scene {
    table: TableIn,
    now: i64,
    aspect: i32,
    drums: [[Ease; 5]; 2],
    lid: [Ease; 2],
    shots: Vec<Shot>,
    pulses: Vec<Pulse>,
    last: render::Finished,
}

// ---- colours: carved woods, painted inlays, brass ----
const fn rgb(r: i32, g: i32, b: i32) -> Rgb {
    [r * ONE / 255, g * ONE / 255, b * ONE / 255]
}
const TABLE: Rgb = rgb(120, 78, 44);
const TABLE_EDGE: Rgb = rgb(88, 54, 30);
const FELT: Rgb = rgb(70, 98, 64);
const CABINET: Rgb = rgb(104, 64, 36);
const SLAT: Rgb = rgb(250, 236, 200);
const SLAT_DIM: Rgb = rgb(170, 146, 110);
const RED: Rgb = rgb(200, 66, 46);
const BLUE: Rgb = rgb(52, 108, 170);
const GILT: Rgb = rgb(226, 178, 64);
const HAMMER: Rgb = rgb(150, 104, 62);
const HANDLE: Rgb = rgb(96, 62, 34);
const PLANK: Rgb = rgb(170, 120, 70);
const DIM: Rgb = rgb(92, 70, 52);
const SKIN: Rgb = rgb(232, 196, 156);
const EYE: Rgb = rgb(40, 26, 20);
const CHEEK: Rgb = rgb(240, 150, 140);
const OAK: Rgb = rgb(112, 76, 44);
const BIRCH: Rgb = rgb(226, 206, 170);
const WALNUT: Rgb = rgb(92, 58, 36);
const STEEL: Rgb = rgb(170, 176, 186);
const LEVELS: [Rgb; 3] = [rgb(176, 112, 60), rgb(200, 204, 212), rgb(230, 186, 70)];
const WALL: Rgb = rgb(78, 52, 36);
const FLOOR: Rgb = rgb(60, 40, 28);
const FLAME: Rgb = rgb(255, 196, 96);
const LID: Rgb = rgb(86, 52, 30);

// ---- layout, in scene units (about a centimetre each) ----
const WHEEL_GAP: i32 = int(33) / 10;
const DRUM_R: i32 = int(16) / 10;
const DRUM_Y: i32 = int(18) / 10;
const DRUM_Z: i32 = int(62) / 10;
const SLAT_W: i32 = int(26) / 10;
const LEGEND_X: i32 = int(125) / 10;
const LEGEND_Z: i32 = int(65) / 10;
const BARRICADE_Z: i32 = int(24) / 10;
const PLANK_H: i32 = int(7) / 10;
const COIN_Z: i32 = int(104) / 10;

fn sgn(side: usize) -> i32 {
    if side == 0 {
        ONE
    } else {
        -ONE
    }
}

fn zs(side: usize, z: i32) -> i32 {
    fx::mul(sgn(side), z)
}

fn wheel_x(w: usize) -> i32 {
    (w as i32 - 2) * WHEEL_GAP
}

/// How far the active panel leans from straight up towards the camera, so
/// it faces the person's seat.
const TILT: i32 = fx::deg(38);

/// The drum angle that brings panel `face` of `panels` to the front.
fn face_angle(face: usize, panels: usize) -> i32 {
    TILT - (face as i32) * (TURN / panels.max(1) as i32)
}

fn seat_colour(seat: &str) -> Rgb {
    if seat == "blue" {
        BLUE
    } else {
        RED
    }
}

/// Where a Legend stands. Both Red Legends sit on the left (T1, from SP-Q13).
fn legend_at(side: usize, slot: usize) -> V3 {
    let x = if slot == 0 { -LEGEND_X } else { LEGEND_X };
    v3(x, 0, zs(side, LEGEND_Z))
}

fn barricade_top(side: usize, planks: i32) -> V3 {
    v3(0, planks * PLANK_H, zs(side, BARRICADE_Z))
}

fn coins_at(side: usize) -> V3 {
    v3(0, int(1) / 2, zs(side, COIN_Z))
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            table: TableIn::default(),
            now: 0,
            aspect: ratio(16, 9),
            drums: [[Ease::default(); 5]; 2],
            lid: [Ease::default(); 2],
            shots: Vec::new(),
            pulses: Vec::new(),
            last: (Vec::new(), Vec::new()),
        }
    }

    /// Canvas size in CSS pixels and the device pixel ratio in hundredths;
    /// returns the drawing buffer's size.
    pub fn resize(&mut self, css_w: i32, css_h: i32, dpr_pct: i32) -> [i32; 2] {
        let (w, h) = (css_w.max(1), css_h.max(1));
        self.aspect = (w as i64 * ONE as i64 / h as i64) as i32;
        let dpr = dpr_pct.clamp(100, 200);
        [w * dpr / 100, h * dpr / 100]
    }

    /// The table as the page shows it now. `spun` says the wheels were just
    /// spun, so the unlocked ones turn to their new faces over `spin_ms`.
    pub fn set_table(&mut self, json: &str, spun: bool, now: i64, spin_ms: i64) -> Result<(), String> {
        let t: TableIn = serde_json::from_str(json).map_err(|e| e.to_string())?;
        self.now = now;
        for (s, side) in t.sides.iter().enumerate().take(2) {
            let before = self.table.sides.get(s);
            let was_covered = before.map(|b| b.wheels.is_empty()).unwrap_or(true);
            for (w, wheel) in side.wheels.iter().enumerate().take(5) {
                let to = face_angle(wheel.face, wheel.panels.len());
                let held = before
                    .and_then(|b| b.wheels.get(w))
                    .map(|x| x.locked)
                    .unwrap_or(false);
                let d = &mut self.drums[s][w];
                let turn_it = (spun && s == 0 && !held) || (s == 1 && was_covered);
                if turn_it {
                    let from = d.at(now);
                    let laps = TURN * (2 + w as i32 / 2);
                    let mut target = to;
                    while target > from - laps {
                        target -= TURN;
                    }
                    *d = Ease {
                        from,
                        to: target,
                        start: now,
                        dur: spin_ms + (w as i64) * spin_ms / 8,
                    };
                } else if !d.moving(now) {
                    *d = Ease {
                        from: to,
                        to,
                        start: now,
                        dur: 0,
                    };
                }
            }
            let covered = side.covered;
            let lid = &mut self.lid[s];
            let open = if covered { 0 } else { ONE };
            if lid.to != open {
                *lid = Ease {
                    from: lid.at(now),
                    to: open,
                    start: now,
                    dur: spin_ms,
                };
            }
        }
        self.table = t;
        Ok(())
    }

    /// Starts the animation for one event of the end of a turn.
    pub fn event(&mut self, json: &str, now: i64, dur: i64) -> Result<(), String> {
        let e: EventIn = serde_json::from_str(json).map_err(|e| e.to_string())?;
        self.now = now;
        match e.kind.as_str() {
            "hit" | "burst" => {
                let from = legend_at(e.side, e.slot).add(v3(0, int(3), 0));
                let foe = &self.table.sides.get(e.foe).cloned().unwrap_or_default();
                let to = if e.target == "barricade" {
                    barricade_top(e.foe, foe.barricade.max(1)).sub(v3(0, PLANK_H / 2, 0))
                } else {
                    coins_at(e.foe)
                };
                let seat = self
                    .table
                    .sides
                    .get(e.side)
                    .and_then(|s| s.legends.get(e.slot))
                    .cloned()
                    .unwrap_or_default();
                let look = match (e.kind.as_str(), seat.prof.as_str()) {
                    ("burst", _) => ShotLook::Burst,
                    (_, "wizard") => ShotLook::Bolt(seat_colour(&seat.seat)),
                    (_, "bowman") => ShotLook::Arrow,
                    _ => ShotLook::Blade(seat_colour(&seat.seat)),
                };
                // the arrow flies at height 3; the Wizard's second bolt arcs over the top
                let peak = match (look, e.over) {
                    (_, true) => int(9),
                    (ShotLook::Arrow, false) => int(3),
                    _ => int(4),
                };
                self.shots.push(Shot {
                    from,
                    to,
                    peak,
                    start: now,
                    dur,
                    look,
                });
            }
            "act" => self.pulses.push(Pulse {
                side: e.side,
                slot: e.slot,
                start: now,
                dur,
                glow: false,
            }),
            "evolve" => self.pulses.push(Pulse {
                side: e.side,
                slot: e.slot,
                start: now,
                dur,
                glow: true,
            }),
            _ => {}
        }
        Ok(())
    }

    /// True while anything is moving, so the page can stop drawing when the
    /// table is still.
    pub fn busy(&self, now: i64) -> bool {
        self.drums.iter().flatten().any(|d| d.moving(now))
            || self.lid.iter().any(|l| l.moving(now))
            || self.shots.iter().any(|s| now < s.start + s.dur + s.dur / 2)
            || self.pulses.iter().any(|p| now < p.start + p.dur)
    }

    fn camera(&self) -> Camera {
        // a narrow screen backs the camera off so the whole table fits
        let wide = ratio(16, 9);
        let back = if self.aspect < wide {
            fx::div(wide, self.aspect.max(ONE / 3))
        } else {
            ONE
        };
        let eye = v3(0, fx::mul(int(27), back), fx::mul(int(26), back) + int(2));
        let look = v3(0, 0, int(1));
        Camera {
            eye,
            fwd: look.sub(eye).norm(),
            fov: fx::deg(38),
            far: int(400),
            up: V3::UP,
        }
    }

    pub fn mats(&self) -> Vec<i32> {
        self.camera().mat_values(self.aspect).to_vec()
    }

    /// Where the middle of each of your wheels' fronts lands on screen, as
    /// percent (Q16) from the left and the top: the page puts the lock
    /// buttons there.
    pub fn anchors(&self) -> Vec<i32> {
        let cam = self.camera();
        let mut out = Vec::new();
        for w in 0..5 {
            let p = v3(wheel_x(w), int(1) / 2, DRUM_Z + int(28) / 10);
            let (x, y) = cam.screen(self.aspect, p).unwrap_or((0, 0));
            out.extend_from_slice(&[x, y]);
        }
        out
    }

    pub fn uniforms(&self) -> Vec<i32> {
        let c = |v: [i32; 3]| [v[0] * ONE / 255, v[1] * ONE / 255, v[2] * ONE / 255];
        let pct = |v: &[i32]| v.iter().map(|x| x * ONE / 100).collect::<Vec<i32>>();
        let dir = v3(-ONE * 45 / 100, ONE * 90 / 100, ONE * 55 / 100).norm();
        let mut out = vec![dir.x, dir.y, dir.z];
        out.extend_from_slice(&c([46, 30, 22])); // sky: the tavern's dark
        out.extend_from_slice(&c([46, 30, 22])); // fog
        out.extend_from_slice(&[int(60), int(140)]);
        out.extend_from_slice(&c([255, 226, 180])); // candle-warm key light
        out.extend_from_slice(&c([40, 24, 16])); // ink
        out.extend_from_slice(&pct(&[5, 1, 12]));
        out.extend_from_slice(&pct(&[40, 30, 75, 55]));
        out.extend_from_slice(&pct(&[44, 40, 42]));
        out.extend_from_slice(&pct(&[34, 24, 18]));
        out.push(1);
        out.push(ONE / 10_000);
        out.extend_from_slice(&[
            int(render::PASS_SOLID as i32),
            int(render::PASS_INK as i32),
            int(render::PASS_TINT as i32),
            ONE / 2,
        ]);
        out.extend_from_slice(&[int(STYLE_GLOW), ONE / 2]);
        out
    }

    pub fn clear_color(&self) -> Vec<i32> {
        c3([46, 30, 22]).to_vec()
    }

    pub fn values(&self) -> Vec<i32> {
        self.last.0.clone()
    }

    pub fn draws(&self) -> Vec<u32> {
        render::draws(&[], &self.last.1)
    }

    /// Builds this frame's instances.
    pub fn render(&mut self, now: i64) {
        self.now = now;
        self.shots.retain(|s| now < s.start + s.dur + s.dur / 2);
        self.pulses.retain(|p| now < p.start + p.dur);
        let mut b = Batch::new();
        self.room(&mut b);
        for side in 0..self.table.sides.len().min(2) {
            self.half(&mut b, side);
        }
        self.flying(&mut b);
        self.last = b.finish();
    }

    fn room(&self, b: &mut Batch) {
        b.push(
            LAYER_PLAIN,
            Mesh::Cube,
            &render::boxed(v3(0, -int(30), 0), v3(int(200), int(2), int(200))),
            FLOOR,
            0,
        );
        b.push(
            LAYER_PLAIN,
            Mesh::Cube,
            &render::boxed(v3(0, int(10), -int(46)), v3(int(160), int(80), int(2))),
            WALL,
            0,
        );
        // the table: top, rim, legs
        b.push(
            LAYER_INKED,
            Mesh::Cube,
            &render::boxed(v3(0, -int(6) / 10, 0), v3(int(36), int(12) / 10, int(26))),
            TABLE,
            0,
        );
        b.push(
            LAYER_INKED,
            Mesh::Cube,
            &render::boxed(v3(0, -int(15) / 10, 0), v3(int(37), int(6) / 10, int(27))),
            TABLE_EDGE,
            0,
        );
        for (x, z) in [(-16, -11), (16, -11), (-16, 11), (16, 11)] {
            let at = v3(int(x), -int(16), int(z));
            b.push(
                LAYER_INKED,
                Mesh::FacetCyl,
                &render::boxed(at, v3(int(2), int(28), int(2))),
                TABLE_EDGE,
                0,
            );
        }
        for side in 0..2 {
            let at = v3(0, int(1) / 50, zs(side, int(6)));
            b.push(
                LAYER_PLAIN,
                Mesh::Cube,
                &render::boxed(at, v3(int(32), int(1) / 20, int(10))),
                FELT,
                0,
            );
        }
        // candles at the corners
        for x in [-int(16), int(16)] {
            let at = v3(x, 0, -int(11));
            b.push(
                LAYER_INKED,
                Mesh::Cyl,
                &render::standing(at, v3(int(1), int(3), int(1))),
                BIRCH,
                0,
            );
            let flame = render::boxed(
                at.add(v3(0, int(36) / 10, 0)),
                v3(int(6) / 10, int(10) / 10, int(6) / 10),
            );
            b.push(LAYER_PLAIN, Mesh::Sphere, &flame, FLAME, STYLE_GLOW);
        }
        // barrels by the wall
        for (x, z) in [(-26, -36), (-21, -38), (24, -37)] {
            let at = v3(int(x), -int(30), int(z));
            b.push(
                LAYER_INKED,
                Mesh::FacetTaper,
                &render::standing(at, v3(int(5), int(7), int(5))),
                OAK,
                0,
            );
            b.push(
                LAYER_INKED,
                Mesh::Ring,
                &render::boxed(at.add(v3(0, int(5), 0)), v3(int(5), int(1) / 2, int(5))),
                DIM,
                0,
            );
        }
    }

    fn half(&self, b: &mut Batch, side: usize) {
        let s = &self.table.sides[side];
        // the cabinet the drums sit in
        let cab = v3(0, int(8) / 10, zs(side, DRUM_Z));
        b.push(
            LAYER_INKED,
            Mesh::Cube,
            &render::boxed(cab, v3(int(17), int(16) / 10, int(4))),
            CABINET,
            0,
        );
        for w in 0..5 {
            self.drum(b, side, w);
        }
        // the cover over the opponent's wheels, lifting at the reveal
        let open = self.lid[side].at(self.now);
        if side == 1 && (s.covered || open < ONE) {
            let lift = fx::mul(open, int(9));
            let at = v3(0, int(29) / 10 + lift, zs(side, DRUM_Z) - fx::mul(open, int(3)));
            let a = Affine::translate(at)
                .then(&Affine::rot_x(fx::mul(open, fx::deg(25))))
                .then(&Affine::scale(v3(int(18), int(24) / 10, int(5))));
            b.push(LAYER_INKED, Mesh::Cube, &a, LID, 0);
        }
        // the Barricade: planks standing between the wheels and the seam
        for i in 0..s.barricade.min(s.barricade_max.max(1)) {
            let at = v3(0, i * PLANK_H + PLANK_H / 2, zs(side, BARRICADE_Z));
            let size = v3(int(10) - i * ONE / 4, PLANK_H - ONE / 20, int(9) / 10);
            b.push(LAYER_INKED, Mesh::Cube, &render::boxed(at, size), PLANK, 0);
        }
        // health: a row of coins in front of each player
        let gap = int(13) / 10;
        for i in 0..s.health_max {
            let x = (i * 2 - (s.health_max - 1)) * gap / 2;
            let at = v3(x, int(2) / 10, zs(side, COIN_Z));
            let colour = if i < s.health { RED } else { DIM };
            let h = if i < s.health { int(4) / 10 } else { int(1) / 10 };
            b.push(
                LAYER_INKED,
                Mesh::FacetCyl,
                &render::standing(at, v3(int(1), h, int(1))),
                colour,
                0,
            );
        }
        for slot in 0..s.legends.len().min(2) {
            self.legend(b, side, slot);
        }
    }

    fn drum(&self, b: &mut Batch, side: usize, w: usize) {
        let s = &self.table.sides[side];
        let centre = v3(wheel_x(w), DRUM_Y, zs(side, DRUM_Z));
        // end caps
        for dx in [-(SLAT_W / 2 + ONE / 10), SLAT_W / 2 + ONE / 10] {
            let a = Affine::translate(centre.add(v3(dx, 0, 0)))
                .then(&Affine::rot_z(fx::QUARTER))
                .then(&Affine::scale(v3(
                    DRUM_R * 2 + ONE / 5,
                    ONE / 5,
                    DRUM_R * 2 + ONE / 5,
                )));
            b.push(LAYER_INKED, Mesh::FacetCyl, &a, CABINET, 0);
        }
        let Some(wheel) = s.wheels.get(w) else {
            // covered: a plain drum under the lid
            let a = Affine::translate(centre)
                .then(&Affine::rot_z(fx::QUARTER))
                .then(&Affine::scale(v3(DRUM_R * 2, SLAT_W, DRUM_R * 2)));
            b.push(LAYER_INKED, Mesh::FacetCyl, &a, SLAT, 0);
            return;
        };
        let n = wheel.panels.len().max(1);
        let angle = self.drums[side][w].at(self.now);
        let step = TURN / n as i32;
        let thick = fx::mul(DRUM_R, fx::mul(step, int(6)));
        for (i, p) in wheel.panels.iter().enumerate() {
            let a_i = angle + i as i32 * step;
            // skip slats facing straight down: the cabinet hides them
            let up = fx::cos(a_i);
            if up < -ONE / 2 {
                continue;
            }
            let spin = Affine::translate(centre).then(&Affine::rot_x(a_i));
            let slat = spin
                .then(&Affine::translate(v3(0, DRUM_R, 0)))
                .then(&Affine::scale(v3(SLAT_W, ONE / 5, thick)));
            b.push(
                LAYER_INKED,
                Mesh::Cube,
                &slat,
                if fx::cos(a_i - TILT) > fx::cos(step / 2) {
                    SLAT
                } else {
                    SLAT_DIM
                },
                0,
            );
            let face = spin.then(&Affine::translate(v3(0, DRUM_R + ONE / 5, 0)));
            if p.gilded {
                let ring = face.then(&Affine::scale(v3(SLAT_W * 3 / 4, ONE / 20, thick * 3 / 4)));
                b.push(LAYER_PLAIN, Mesh::FacetCyl, &ring, GILT, 0);
            }
            let count = p.count.clamp(1, 2) as i32;
            for k in 0..count {
                let dx = if count == 1 { 0 } else { (k * 2 - 1) * SLAT_W / 4 };
                let at = face.then(&Affine::translate(v3(dx, ONE / 6, 0)));
                symbol(b, &at, &p.sym);
            }
        }
        // the window: two rails across the drum, above and below the active panel
        let half = fx::mul(DRUM_R, fx::cos(step / 2));
        for k in [-1, 1] {
            let a = Affine::translate(centre)
                .then(&Affine::rot_x(TILT + k * step / 2))
                .then(&Affine::translate(v3(0, half + ONE / 4, 0)))
                .then(&Affine::scale(v3(SLAT_W + ONE / 3, ONE / 7, ONE / 7)));
            b.push(
                LAYER_INKED,
                Mesh::Cube,
                &a,
                if wheel.locked { GILT } else { EYE },
                0,
            );
        }
        if side == 0 && wheel.locked {
            // a brass padlock in front of a locked wheel
            let at = v3(wheel_x(w), int(9) / 10, zs(side, DRUM_Z + int(22) / 10));
            b.push(
                LAYER_INKED,
                Mesh::Cube,
                &render::boxed(at, v3(int(8) / 10, int(7) / 10, int(4) / 10)),
                GILT,
                0,
            );
            let shackle = Affine::translate(at.add(v3(0, int(5) / 10, 0)))
                .then(&Affine::rot_x(fx::QUARTER))
                .then(&Affine::scale(v3(int(6) / 10, int(2) / 10, int(6) / 10)));
            b.push(LAYER_INKED, Mesh::Ring, &shackle, STEEL, 0);
        }
    }

    fn legend(&self, b: &mut Batch, side: usize, slot: usize) {
        let l = &self.table.sides[side].legends[slot];
        let mut at = legend_at(side, slot);
        let seat = seat_colour(&l.seat);
        for p in self.pulses.iter().filter(|p| p.side == side && p.slot == slot) {
            let t = (((self.now - p.start).max(0) * ONE as i64) / p.dur.max(1)) as i32;
            let hop = fx::sin(fx::mul(t, fx::HALF));
            if p.glow {
                let r = int(3) + fx::mul(t, int(3));
                let glow = render::boxed(at.add(v3(0, int(2), 0)), v3(r, r, r));
                b.tint(Mesh::Sphere, &glow, GILT, ONE / 2 - t / 2);
            } else {
                at = at.add(v3(0, fx::mul(hop, int(12) / 10), 0));
            }
        }
        let level = LEVELS[(l.level as usize).min(2)];
        b.push(
            LAYER_INKED,
            Mesh::FacetCyl,
            &render::standing(at, v3(int(30) / 10, int(5) / 10, int(30) / 10)),
            level,
            0,
        );
        b.push(
            LAYER_PLAIN,
            Mesh::Ring,
            &render::boxed(
                at.add(v3(0, int(3) / 10, 0)),
                v3(int(31) / 10, int(2) / 10, int(31) / 10),
            ),
            seat,
            0,
        );
        let wood = match l.prof.as_str() {
            "wizard" => BIRCH,
            "bowman" => WALNUT,
            _ => OAK,
        };
        let body = at.add(v3(0, int(5) / 10, 0));
        b.push(
            LAYER_INKED,
            Mesh::Taper,
            &render::standing(body, v3(int(22) / 10, int(24) / 10, int(22) / 10)),
            seat,
            0,
        );
        b.push(
            LAYER_INKED,
            Mesh::Cyl,
            &render::standing(
                body.add(v3(0, int(13) / 10, 0)),
                v3(int(19) / 10, int(4) / 10, int(19) / 10),
            ),
            wood,
            0,
        );
        let head = body.add(v3(0, int(36) / 10, 0));
        b.push(
            LAYER_INKED,
            Mesh::Sphere,
            &render::boxed(head, v3(int(21) / 10, int(20) / 10, int(21) / 10)),
            SKIN,
            0,
        );
        for dx in [-int(4) / 10, int(4) / 10] {
            b.push(
                LAYER_PLAIN,
                Mesh::Sphere,
                &render::boxed(
                    head.add(v3(dx, 0, int(96) / 100)),
                    v3(int(2) / 10, int(3) / 10, int(1) / 10),
                ),
                EYE,
                0,
            );
            b.tint(
                Mesh::Sphere,
                &render::boxed(
                    head.add(v3(dx * 2, -int(3) / 10, int(85) / 100)),
                    v3(int(4) / 10, int(2) / 10, int(1) / 10),
                ),
                CHEEK,
                ONE * 6 / 10,
            );
        }
        match l.prof.as_str() {
            "wizard" => {
                let hat = Affine::translate(head.add(v3(0, int(9) / 10, 0)))
                    .then(&Affine::rot_z(fx::deg(-8)))
                    .then(&Affine::scale(v3(int(22) / 10, int(26) / 10, int(22) / 10)));
                b.push(
                    LAYER_INKED,
                    Mesh::FacetCone,
                    &hat.then(&Affine::translate(v3(0, ONE / 2, 0))),
                    seat,
                    0,
                );
                b.push(
                    LAYER_INKED,
                    Mesh::Cyl,
                    &render::boxed(
                        head.add(v3(0, int(8) / 10, 0)),
                        v3(int(26) / 10, int(2) / 10, int(26) / 10),
                    ),
                    wood,
                    0,
                );
                let staff = at.add(v3(int(16) / 10, 0, int(6) / 10));
                b.push(
                    LAYER_INKED,
                    Mesh::Cyl,
                    &render::standing(staff, v3(int(3) / 10, int(52) / 10, int(3) / 10)),
                    WALNUT,
                    0,
                );
                b.push(
                    LAYER_PLAIN,
                    Mesh::Sphere,
                    &render::boxed(
                        staff.add(v3(0, int(56) / 10, 0)),
                        v3(int(7) / 10, int(7) / 10, int(7) / 10),
                    ),
                    seat,
                    STYLE_GLOW,
                );
            }
            "bowman" => {
                b.push(
                    LAYER_INKED,
                    Mesh::Sphere,
                    &render::boxed(
                        head.add(v3(0, int(2) / 10, -int(3) / 10)),
                        v3(int(24) / 10, int(23) / 10, int(22) / 10),
                    ),
                    wood,
                    0,
                );
                let bow = Affine::translate(at.add(v3(int(17) / 10, int(26) / 10, int(5) / 10)))
                    .then(&Affine::rot_z(fx::QUARTER))
                    .then(&Affine::scale(v3(int(36) / 10, int(3) / 10, int(14) / 10)));
                b.push(LAYER_INKED, Mesh::Ring, &bow, WALNUT, 0);
            }
            _ => {
                b.push(
                    LAYER_INKED,
                    Mesh::Sphere,
                    &render::boxed(
                        head.add(v3(0, int(3) / 10, -int(1) / 10)),
                        v3(int(23) / 10, int(19) / 10, int(23) / 10),
                    ),
                    STEEL,
                    0,
                );
                b.push(
                    LAYER_INKED,
                    Mesh::Cube,
                    &render::boxed(
                        head.add(v3(0, int(12) / 10, 0)),
                        v3(int(3) / 10, int(6) / 10, int(16) / 10),
                    ),
                    seat,
                    0,
                );
                let sword = Affine::translate(at.add(v3(int(17) / 10, int(30) / 10, int(5) / 10)))
                    .then(&Affine::rot_z(fx::deg(-12)))
                    .then(&Affine::scale(v3(int(3) / 10, int(36) / 10, int(1) / 10)));
                b.push(LAYER_INKED, Mesh::Cube, &sword, STEEL, 0);
                let shield = Affine::translate(at.add(v3(-int(14) / 10, int(20) / 10, int(9) / 10)))
                    .then(&Affine::rot_x(fx::QUARTER))
                    .then(&Affine::scale(v3(int(16) / 10, int(2) / 10, int(18) / 10)));
                b.push(LAYER_INKED, Mesh::FacetCyl, &shield, seat, 0);
            }
        }
        // the energy bar: a column of segments beside the Legend
        let out = if slot == 0 { -int(26) / 10 } else { int(26) / 10 };
        for i in 0..l.cost {
            let c = if i < l.energy { seat } else { DIM };
            let p = at.add(v3(out, i * int(7) / 10 + int(4) / 10, 0));
            b.push(
                LAYER_INKED,
                Mesh::Cube,
                &render::boxed(p, v3(int(8) / 10, int(5) / 10, int(8) / 10)),
                c,
                0,
            );
        }
        // XP beads along the front of the plinth
        for i in 0..l.xp_max {
            let x = (i * 2 - (l.xp_max - 1)) * int(4) / 10;
            let c = if i < l.xp { GILT } else { DIM };
            let p = at.add(v3(x, int(3) / 10, int(17) / 10));
            b.push(
                LAYER_PLAIN,
                Mesh::Sphere,
                &render::boxed(p, v3(int(4) / 10, int(4) / 10, int(4) / 10)),
                c,
                0,
            );
        }
    }

    fn flying(&self, b: &mut Batch) {
        for s in &self.shots {
            let t = (((self.now - s.start).max(0) * ONE as i64) / s.dur.max(1)) as i32;
            if t >= ONE {
                // the impact: a flash that fades
                let k = (t - ONE) * 2;
                let r = int(1) + fx::mul(k, int(2));
                b.tint(
                    Mesh::Sphere,
                    &render::boxed(s.to, v3(r, r, r)),
                    FLAME,
                    (ONE - k).max(0) * 7 / 10,
                );
                continue;
            }
            let arc = fx::mul(fx::mul(int(4), t), ONE - t);
            let p = s.from.lerp(s.to, t).add(v3(0, fx::mul(s.peak, arc), 0));
            let dir = s.to.sub(s.from).norm();
            match s.look {
                ShotLook::Bolt(c) => b.push(
                    LAYER_PLAIN,
                    Mesh::Sphere,
                    &render::boxed(p, v3(int(1), int(1), int(1))),
                    c,
                    STYLE_GLOW,
                ),
                ShotLook::Burst => b.push(
                    LAYER_PLAIN,
                    Mesh::Facet,
                    &render::boxed(p, v3(int(14) / 10, int(14) / 10, int(14) / 10)),
                    GILT,
                    STYLE_GLOW,
                ),
                ShotLook::Arrow => {
                    let a = Affine::translate(p)
                        .then(&Affine::looking(dir))
                        .then(&Affine::scale(v3(int(2) / 10, int(2) / 10, int(26) / 10)));
                    b.push(LAYER_INKED, Mesh::Cube, &a, WALNUT, 0);
                }
                ShotLook::Blade(c) => {
                    let spin = fx::mul(t, TURN * 2);
                    let a = Affine::translate(p)
                        .then(&Affine::looking(dir))
                        .then(&Affine::rot_z(spin))
                        .then(&Affine::scale(v3(int(4) / 10, int(18) / 10, int(4) / 10)));
                    b.push(LAYER_INKED, Mesh::Octa, &a, c, 0);
                }
            }
        }
    }
}

fn c3(v: [i32; 3]) -> [i32; 3] {
    [v[0] * ONE / 255, v[1] * ONE / 255, v[2] * ONE / 255]
}

/// One carved symbol on a slat: a square for red, a diamond for blue, a
/// hammer for the Barricade (the same shapes as the 2D views).
fn symbol(b: &mut Batch, at: &Affine, sym: &str) {
    match sym {
        "red" => b.push(
            LAYER_INKED,
            Mesh::Cube,
            &at.then(&Affine::scale(v3(int(7) / 10, int(3) / 10, int(7) / 10))),
            RED,
            0,
        ),
        "blue" => {
            let a = at.then(&Affine::rot_y(fx::QUARTER / 2)).then(&Affine::scale(v3(
                int(8) / 10,
                int(4) / 10,
                int(8) / 10,
            )));
            b.push(LAYER_INKED, Mesh::Octa, &a, BLUE, 0);
        }
        _ => {
            b.push(
                LAYER_INKED,
                Mesh::Cube,
                &at.then(&Affine::translate(v3(0, 0, -int(2) / 10)))
                    .then(&Affine::scale(v3(int(8) / 10, int(3) / 10, int(3) / 10))),
                HAMMER,
                0,
            );
            b.push(
                LAYER_INKED,
                Mesh::Cube,
                &at.then(&Affine::translate(v3(0, 0, int(2) / 10)))
                    .then(&Affine::scale(v3(int(2) / 10, int(2) / 10, int(6) / 10))),
                HANDLE,
                0,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TABLE_JSON: &str = r#"{"sides":[
      {"health":10,"health_max":10,"barricade":2,"barricade_max":5,"covered":false,
       "legends":[{"seat":"red","prof":"knight","energy":1,"cost":3,"xp":0,"xp_max":6,"level":0},
                  {"seat":"blue","prof":"wizard","energy":0,"cost":5,"xp":0,"xp_max":6,"level":0}],
       "wheels":[{"face":3,"locked":true,"panels":[{"sym":"red","count":1,"gilded":false},{"sym":"blue","count":2,"gilded":true}]}]},
      {"health":7,"health_max":10,"barricade":0,"barricade_max":5,"covered":true,
       "legends":[{"seat":"red","prof":"bowman","energy":0,"cost":4,"xp":0,"xp_max":6,"level":2},
                  {"seat":"blue","prof":"knight","energy":0,"cost":3,"xp":0,"xp_max":6,"level":0}],
       "wheels":[]}]}"#;

    #[test]
    fn the_scene_draws_what_the_table_says() {
        let mut s = Scene::new();
        s.set_table(TABLE_JSON, false, 0, 500).unwrap();
        s.render(0);
        let before = s.values().len();
        let mut fewer = TABLE_JSON.replace("\"health\":7", "\"health\":3");
        fewer = fewer.replace("\"barricade\":2", "\"barricade\":0");
        s.set_table(&fewer, false, 0, 500).unwrap();
        s.render(0);
        // two planks fewer; lost coins are still drawn, flat
        assert_eq!(before - s.values().len(), 2 * render::INSTANCE_VALUES);
    }

    #[test]
    fn a_spun_wheel_eases_to_its_face_and_then_stops() {
        let mut s = Scene::new();
        let unlocked = TABLE_JSON.replace("\"locked\":true", "\"locked\":false");
        s.set_table(&unlocked, false, 0, 500).unwrap();
        s.set_table(&unlocked.replace("\"face\":3", "\"face\":1"), true, 0, 500)
            .unwrap();
        assert!(s.busy(100));
        assert!(!s.busy(10_000));
        let d = s.drums[0][0];
        assert_eq!(fx::wrap(d.at(10_000)), fx::wrap(face_angle(1, 2)));
    }

    #[test]
    fn the_cover_lifts_when_the_core_uncovers_the_opponent() {
        let mut s = Scene::new();
        s.set_table(TABLE_JSON, false, 0, 500).unwrap();
        assert_eq!(s.lid[1].at(0), 0);
        s.set_table(
            &TABLE_JSON.replace("\"covered\":true", "\"covered\":false"),
            false,
            0,
            500,
        )
        .unwrap();
        assert_eq!(s.lid[1].at(1000), ONE);
    }

    /// Both sides uncovered, every wheel showing all ten panels as doubles:
    /// the most the table ever draws. One instanced draw per mesh and pass
    /// keeps the draw calls few however many instances there are.
    #[test]
    fn a_full_table_stays_within_the_instance_budget() {
        let panel = r#"{"sym":"red","count":2,"gilded":true}"#;
        let wheel = format!(
            r#"{{"face":0,"locked":true,"panels":[{}]}}"#,
            [panel; 10].join(",")
        );
        let wheels = [wheel.as_str(); 5].join(",");
        let legend = r#"{"seat":"red","prof":"wizard","energy":2,"cost":5,"xp":3,"xp_max":6,"level":2}"#;
        let side = format!(
            r#"{{"health":10,"health_max":10,"barricade":5,"barricade_max":5,"covered":false,"legends":[{legend},{legend}],"wheels":[{wheels}]}}"#
        );
        let mut s = Scene::new();
        s.set_table(
            &format!(r#"{{"sides":[{side},{side}]}}"#, side = side),
            false,
            0,
            500,
        )
        .unwrap();
        s.render(0);
        let instances = s.values().len() / render::INSTANCE_VALUES;
        let draws = s.draws().len() / render::DRAW_VALUES;
        assert!(instances < 1500, "{instances} instances");
        assert!(draws < 40, "{draws} draws");
    }

    #[test]
    fn lock_anchors_sit_left_to_right_across_the_screen() {
        let s = Scene::new();
        let a = s.anchors();
        assert_eq!(a.len(), 10);
        assert!(a[0] < a[2] && a[2] < a[4] && a[4] < a[6] && a[6] < a[8]);
        assert!(a.iter().all(|&v| (0..=int(100)).contains(&v)));
    }
}
