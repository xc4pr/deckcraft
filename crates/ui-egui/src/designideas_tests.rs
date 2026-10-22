//! Design Ideas: the ribbon button stays reachable in a narrow window (#1202).

use deckcraft_engine::Session;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use serde_json::json;

use crate::{Services, SlideApp};

fn harness(tab: &str, width: f32) -> Harness<'static, SlideApp> {
    let mut s = Session::with_new();
    s.execute("slide.new", &json!({"layout": "title", "title": "Hello"})).unwrap();
    let mut app = SlideApp::new(s, Services::default());
    app.ui.tab = tab.into();
    let mut h = Harness::builder().with_size(egui::vec2(width, 800.0)).with_step_dt(1.0 / 60.0).build_ui_state(
        |ui, app: &mut SlideApp| {
            app.logic(ui.ctx());
            app.ui(ui);
        },
        app,
    );
    h.run_steps(3);
    h
}

#[test]
fn design_ideas_button_is_on_screen_in_a_1280_px_window_and_opens_the_pane() {
    for tab in ["Home", "Design"] {
        let mut h = harness(tab, 1280.0);
        let b = h.get_by_label("Design Ideas");
        let r = b.rect();
        assert!(r.max.x <= 1280.0 - 8.0, "{tab}: the button is clipped off the ribbon at x {}..{}", r.min.x, r.max.x);
        b.click();
        h.run_steps(3);
        assert_eq!(h.state().ui.pane.as_deref(), Some("designer"), "{tab}");
    }
}
