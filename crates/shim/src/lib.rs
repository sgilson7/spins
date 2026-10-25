//! tool_shim: moves values between the page and `tool_core::api` (the
//! rules) and `spins_view3d::scene` (the 3D view's instances).
//!
//! Builder requirement: the shim decides nothing. Each function is one call
//! into a crate and a conversion of its arguments or result. The 3D scene
//! speaks Q16 integers; the values WebGL wants as floats (matrices,
//! uniforms, the clear colour, screen anchors) are divided by `ONE` here.
//! `crates/core/tests/boundary.rs` fails if a branch, a loop or the `?`
//! operator appears in this file.
//!
//! All three views load the module this crate builds (SP-1). The 2D views
//! never call the 3D methods.

use spins_view3d::{fx, scene};
use tool_core::api;
use wasm_bindgen::prelude::*;

const ONE: f32 = fx::ONE as f32;

fn floats(v: Vec<i32>) -> Vec<f32> {
    v.into_iter().map(|x| x as f32 / ONE).collect()
}

#[wasm_bindgen]
pub struct Game {
    session: api::Session,
    scene: scene::Scene,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(strings: &str, glossary: &str, save: &str, seed: u32) -> Result<Game, JsValue> {
        api::Session::new(strings, glossary, save, seed)
            .map(|session| Game {
                session,
                scene: scene::Scene::new(),
            })
            .map_err(|e| JsValue::from_str(&e))
    }

    pub fn act(&mut self, name: &str, value: &str) -> String {
        self.session.act(name, value)
    }

    pub fn ui(&self) -> String {
        self.session.ui_json()
    }

    // ---- the 3D view only ----

    pub fn resize(&mut self, css_w: f64, css_h: f64, dpr: f64) -> Vec<i32> {
        self.scene
            .resize(css_w as i32, css_h as i32, (dpr * 100.0) as i32)
            .to_vec()
    }

    pub fn set_table(&mut self, table_json: &str, spun: bool, now_ms: f64, spin_ms: f64) -> String {
        api_answer(
            self.scene
                .set_table(table_json, spun, now_ms as i64, spin_ms as i64),
        )
    }

    pub fn event(&mut self, event_json: &str, now_ms: f64, dur_ms: f64) -> String {
        api_answer(self.scene.event(event_json, now_ms as i64, dur_ms as i64))
    }

    pub fn busy(&self, now_ms: f64) -> bool {
        self.scene.busy(now_ms as i64)
    }

    pub fn render(&mut self, now_ms: f64) {
        self.scene.render(now_ms as i64)
    }

    pub fn values(&self) -> Vec<i32> {
        self.scene.values()
    }

    pub fn draws(&self) -> Vec<u32> {
        self.scene.draws()
    }

    pub fn mats(&self) -> Vec<f32> {
        floats(self.scene.mats())
    }

    pub fn uniforms(&self) -> Vec<f32> {
        floats(self.scene.uniforms())
    }

    pub fn clear_color(&self) -> Vec<f32> {
        floats(self.scene.clear_color())
    }

    /// Your five wheels' lock-button spots, as percent across and down.
    pub fn anchors(&self) -> Vec<f32> {
        floats(self.scene.anchors())
    }

    pub fn mesh(&self, id: u32) -> Vec<i32> {
        spins_view3d::render::mesh_data(id)
    }
}

/// Sizes and layout the 3D page needs to upload and draw.
#[wasm_bindgen]
pub fn numbers() -> String {
    spins_view3d::render::numbers()
}

fn api_answer(r: Result<(), String>) -> String {
    r.err().unwrap_or_default()
}
