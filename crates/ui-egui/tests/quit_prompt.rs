//! Quitting with unsaved workbooks: the window close asks about every dirty
//! workbook first (the `saveChanges` prompt that File › Close has always had),
//! instead of discarding the changes silently.

use egui_kittest::kittest::Queryable as _;
use gridcraft_engine::{Session, core::CellRef, core::Value};
use gridcraft_ui_egui::{Services, SheetApp};

fn harness(services: Services) -> egui_kittest::Harness<'static, SheetApp> {
    let mut session = Session::new();
    session.new_workbook();
    let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_ui_state(
        |ui, app: &mut SheetApp| {
            let ctx = ui.ctx().clone();
            app.logic(&ctx);
            app.ui(ui);
        },
        SheetApp::new(session, services),
    );
    h.run_steps(4);
    h
}

fn click(h: &mut egui_kittest::Harness<'static, SheetApp>, label: &str) {
    h.get_by_role_and_label(egui::accesskit::Role::Button, label).click();
    h.run_steps(3);
}

fn dialog<'a>(h: &'a egui_kittest::Harness<'static, SheetApp>) -> Option<&'a str> {
    h.state().dialog.as_ref().map(|d| d.name())
}

fn dirty(h: &mut egui_kittest::Harness<'static, SheetApp>, input: &str) {
    h.state_mut().run("cell.set", serde_json::json!({"cell": "A1", "input": input})).unwrap();
    assert!(h.state().session.active().is_some_and(|d| d.is_dirty()), "the edit must leave the workbook unsaved");
}

#[test]
fn a_clean_session_lets_the_window_close_at_once() {
    let mut h = harness(Services::default());
    assert!(h.state_mut().begin_quit(), "nothing to save: the close goes ahead");
    assert!(h.state().quit_ready());
    assert_eq!(dialog(&h), None);
}

#[test]
fn a_dirty_workbook_vetoes_the_close_and_asks_first() {
    let mut h = harness(Services::default());
    dirty(&mut h, "42");
    assert!(!h.state_mut().begin_quit(), "the close is vetoed while the prompt is unanswered");
    h.run_steps(2);
    assert_eq!(dialog(&h), Some("saveChanges"), "the usual save prompt opens");
    assert!(!h.state().quit_ready());
    assert_eq!(h.state().session.documents().len(), 1, "the workbook is still open");
}

#[test]
fn dont_save_lets_the_quit_proceed() {
    let mut h = harness(Services::default());
    dirty(&mut h, "42");
    let uid = h.state().session.active().map(|d| d.uid);
    assert!(!h.state_mut().begin_quit());
    h.run_steps(2);
    click(&mut h, "Don't Save");
    assert!(h.state().quit_ready(), "the answered prompt releases the close");
    assert!(
        h.state().session.documents().iter().all(|d| Some(d.uid) != uid || !d.is_dirty()),
        "the discarded workbook is gone (or was replaced by a fresh blank one)"
    );
}

#[test]
fn cancel_aborts_the_quit_and_a_later_close_asks_again() {
    let mut h = harness(Services::default());
    dirty(&mut h, "42");
    assert!(!h.state_mut().begin_quit());
    h.run_steps(2);
    click(&mut h, "Cancel");
    assert!(!h.state().quit_ready(), "Cancel keeps the app open");
    assert_eq!(dialog(&h), None, "the prompt itself is dismissed");
    assert_eq!(h.state().session.documents().len(), 1);
    assert!(h.state().session.active().is_some_and(|d| d.is_dirty()), "the changes are untouched");
    // The quit is not wedged: asking to close again prompts again.
    assert!(!h.state_mut().begin_quit());
    h.run_steps(2);
    assert_eq!(dialog(&h), Some("saveChanges"));
}

#[test]
fn several_dirty_workbooks_are_asked_about_one_by_one() {
    let mut h = harness(Services::default());
    dirty(&mut h, "1");
    h.state_mut().run("file.new", serde_json::json!({})).unwrap();
    dirty(&mut h, "2");
    assert_eq!(h.state().session.documents().len(), 2);
    assert!(!h.state_mut().begin_quit());
    h.run_steps(2);
    assert_eq!(dialog(&h), Some("saveChanges"), "the first workbook is asked about");
    click(&mut h, "Don't Save");
    assert!(!h.state().quit_ready(), "one workbook is still unanswered");
    assert_eq!(dialog(&h), Some("saveChanges"), "the second workbook gets its own prompt");
    click(&mut h, "Don't Save");
    assert!(h.state().quit_ready(), "both answered: the quit proceeds");
}

#[test]
fn save_through_the_prompt_writes_the_file_and_quits() {
    let dir = std::env::temp_dir().join(format!("gridcraft-quit-save-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let target = dir.join("Saved.xlsx");
    let pick = target.clone();
    let mut h = harness(Services { pick_save: Some(Box::new(move |_| Some(pick.to_string_lossy().into_owned()))), ..Default::default() });
    dirty(&mut h, "42");
    assert!(!h.state_mut().begin_quit());
    h.run_steps(2);
    assert_eq!(dialog(&h), Some("saveChanges"));
    click(&mut h, "Save");
    assert!(h.state().quit_ready(), "saved and closed: the quit proceeds");
    let bytes = std::fs::read(&target).unwrap();
    let (wb, _) = gridcraft_engine::io::open_bytes("Saved.xlsx", &bytes).unwrap();
    assert_eq!(wb.active().unwrap().value(CellRef::new(0, 0)), Value::Number(42.0), "the prompt's Save wrote the edit");
    let _ = std::fs::remove_dir_all(&dir);
}
