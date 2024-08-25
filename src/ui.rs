use adw::{gio, glib, prelude::*, Application, ApplicationWindow, EntryRow, HeaderBar, PreferencesDialog, PreferencesGroup, PreferencesPage, SpinRow, ToolbarView};
use gtk::{DrawingArea, MenuButton, ShortcutsGroup, ShortcutsSection, ShortcutsShortcut, ShortcutsWindow};
use std::{sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}, time::Duration};
use crate::game::Rule;

use super::game::{Game, Cell::*};

pub fn build_ui(app: &Application) {
    let speed_row = SpinRow::with_range(0., 120., 1.);
    speed_row.set_value(30.);
    speed_row.set_title("Evolution speed");
    
    let size_row = SpinRow::with_range(0., 600., 1.);
    size_row.set_value(30.);
    size_row.set_title("Grid size");
    
    let rule_row = EntryRow::builder().title("Rule (Bx/Sy)").text("B3/S23").build();
    let rule = Rule::try_from(rule_row.text().as_str()).unwrap_or_default();
    
    let game = Arc::new(Mutex::new(Game::builder().rule(rule).random_grid(size_row.value() as usize).build()));
    let is_running = Arc::new(AtomicBool::new(true));
    let drawing_area = DrawingArea::new();
    drawing_area.set_cursor_from_name(match !is_running.load(Ordering::Acquire) {
        true => Some("pointer"),
        false => None
    });
    
    drawing_area.set_draw_func({
        let game = Arc::clone(&game);
        move |_, context, width, height| {
            if let Ok(game_guard) = game.lock() {
                let cell_width = width as f64 / game_guard.grid_size() as f64;
                let cell_height = height as f64 / game_guard.grid_size() as f64;
                
                for row in 0..game_guard.grid_size() {
                    for col in 0..game_guard.grid_size() {
                        if let Some(&Live) = game_guard.cell(row, col) {
                            context.set_source_rgb(51. / 255., 209. / 255., 122. / 255.);
                            context.rectangle(row as f64 * cell_width, col as f64 * cell_height, cell_width, cell_height);
                            let _ = context.fill();
                        }
                    }
                }
            }
        }
    });
    
    let gesture = gtk::GestureClick::new();
    gesture.connect_pressed({
        let drawing_area = drawing_area.clone();
        let is_running = Arc::clone(&is_running);
        let game = Arc::clone(&game);
        move |_, _, x, y| {
            if !is_running.load(Ordering::Acquire) {
                if let Ok(mut game_guard) = game.lock() {
                    let grid_size = game_guard.grid_size();
                    let row = (x * grid_size as f64 / drawing_area.width() as f64) as usize;
                    let col = (y * grid_size as f64 / drawing_area.height() as f64) as usize;
                    
                    game_guard.toggle_cell(row, col);
                }
                drawing_area.queue_draw();
            }
        }
    });
    drawing_area.add_controller(gesture);
    
    let toggle_running_action = gio::SimpleAction::new("toggle_running", None);
    let randomize_grid_action = gio::SimpleAction::new("randomize_grid", None);
    let clear_grid_action = gio::SimpleAction::new("clear_grid", None);
    let evolve_action = gio::SimpleAction::new("evolve", None);
    let show_help_overlay_action = gio::SimpleAction::new("show_help_overlay", None);
    let show_preferences_action = gio::SimpleAction::new("show_preferences", None);
    evolve_action.set_enabled(!is_running.load(Ordering::Acquire));

    toggle_running_action.connect_activate({
        let is_running = Arc::clone(&is_running);
        let evolve_action = evolve_action.clone();
        let drawing_area = drawing_area.clone();
        move |_, _| {
            // fetch_xor returns the previous value
            let is_stopped = is_running.fetch_xor(true, Ordering::AcqRel);
            
            evolve_action.set_enabled(is_stopped);
            drawing_area.set_cursor_from_name(match is_stopped {
                true => Some("pointer"),
                false => None
            })
        }
    });
    randomize_grid_action.connect_activate({
        let game = Arc::clone(&game);
        let drawing_area = drawing_area.clone();
        move |_, _| {
            if let Ok(mut game_guard) = game.lock() {
                game_guard.randomize_grid();
            }
            drawing_area.queue_draw();
        }
    });
    clear_grid_action.connect_activate({
        let game = Arc::clone(&game);
        let drawing_area = drawing_area.clone();
        move |_, _| {
            if let Ok(mut game_guard) = game.lock() {
                game_guard.clear_grid();
            }
            drawing_area.queue_draw();
        }
    });
    evolve_action.connect_activate({
        let game = Arc::clone(&game);
        let drawing_area = drawing_area.clone();
        move |_, _| {
            if let Ok(mut game_guard) = game.lock() {
                game_guard.evolve();
            }
            drawing_area.queue_draw();
        }
    });
    
    app.add_action(&toggle_running_action);
    app.add_action(&randomize_grid_action);
    app.add_action(&clear_grid_action);
    app.add_action(&evolve_action);
    app.add_action(&show_help_overlay_action);
    app.add_action(&show_preferences_action);

    app.set_accels_for_action("app.toggle_running", &["space"]);
    app.set_accels_for_action("app.randomize_grid", &["<Ctrl>r"]);
    app.set_accels_for_action("app.clear_grid", &["<Ctrl>e"]);
    app.set_accels_for_action("app.evolve", &["Right"]);
    app.set_accels_for_action("app.show_help_overlay", &["<Ctrl>question"]);
    app.set_accels_for_action("app.show_preferences", &["<Ctrl>comma"]);
    
    let shortcuts_window = ShortcutsWindow::builder().build();
    let shortcuts_section = ShortcutsSection::builder().build();
    let shortcuts_group = ShortcutsGroup::builder().title("General").build();
    
    shortcuts_window.add_section(&shortcuts_section);
    shortcuts_section.add_group(&shortcuts_group);
    
    shortcuts_group.add_shortcut(&ShortcutsShortcut::builder().title("Toggle Running").action_name("app.toggle_running").accelerator("space").build());
    shortcuts_group.add_shortcut(&ShortcutsShortcut::builder().title("Evolve Step").action_name("app.evolve").accelerator("Right").build());
    shortcuts_group.add_shortcut(&ShortcutsShortcut::builder().title("Randomize Grid").action_name("app.randomize_grid").accelerator("<Ctrl>r").build());
    shortcuts_group.add_shortcut(&ShortcutsShortcut::builder().title("Clear Grid").action_name("app.clear_grid").accelerator("<Ctrl>e").build());
    shortcuts_group.add_shortcut(&ShortcutsShortcut::builder().title("Show preferences").action_name("app.show_preferences").accelerator("<Ctrl>comma").build());
    shortcuts_group.add_shortcut(&ShortcutsShortcut::builder().title("Show shortcuts").action_name("app.show_help_overlay").accelerator("<Ctrl>question").build());

    show_help_overlay_action.connect_activate({
        move |_, _| {
            shortcuts_window.present();
        }
    });
    
    let preferences_dialog = PreferencesDialog::new();
    let preferences_page = PreferencesPage::new();
    let preferences_group = PreferencesGroup::new();
    
    preferences_dialog.add(&preferences_page);
    preferences_page.add(&preferences_group);
    preferences_group.add(&speed_row);
    preferences_group.add(&size_row);
    preferences_group.add(&rule_row);
    
    size_row.connect_value_notify({
       let game = Arc::clone(&game);
       let drawing_area = drawing_area.clone();
       move |spin| {
           if let Ok(mut game_guard) = game.lock() {
               game_guard.resize_grid(spin.value() as usize);
           }
           drawing_area.queue_draw();
       }
    });
    
    rule_row.connect_entry_activated({
       let game = Arc::clone(&game);
       let drawing_area = drawing_area.clone();
       move |entry| {
           if let Ok(mut game_guard) = game.lock() {
               let rule = Rule::try_from(entry.text().as_str()).unwrap_or_default();
               game_guard.set_rule(rule);
           }
           drawing_area.queue_draw();
       }
    });
    
    let menu = gio::Menu::new();
    menu.append(Some("_Preferences"), Some("app.show_preferences"));
    menu.append(Some("_Keyboard Shortcuts"), Some("app.show_help_overlay"));

    let menu_button = MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .menu_model(&menu)
        .build();

    let header_bar = HeaderBar::new();
    header_bar.pack_start(&menu_button);
    
    let content = ToolbarView::builder()
        .content(&drawing_area)
        .build();
    content.add_top_bar(&header_bar);
    
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Life")
        .default_width(400)
        .default_height(400)
        .content(&content)
        .build();
    
    show_preferences_action.connect_activate({
        let window = window.clone();
        move |_, _| {
            preferences_dialog.present(Some(&window));
        }
    });
    
    let timeout = Arc::new(Mutex::new(Some(glib::timeout_add_local(Duration::from_millis(1000 / speed_row.value() as u64), {
        let game = Arc::clone(&game);
        let drawing_area = drawing_area.clone();
        let is_running = Arc::clone(&is_running);
        move || {
            if is_running.load(Ordering::Acquire) {
                if let Ok(mut game_guard) = game.lock() {
                    game_guard.evolve();
                }
                drawing_area.queue_draw();
            }
            glib::ControlFlow::Continue
        }
    }))));
    
    speed_row.connect_value_notify({
        let game = Arc::clone(&game);
        let drawing_area = drawing_area.clone();
        let is_running = Arc::clone(&is_running);
        let timeout = Arc::clone(&timeout);
        move |spin| {
            if let Ok(mut timeout_guard) = timeout.lock() {
                if let Some(src_id) = timeout_guard.take() {
                    src_id.remove();
                }
                *timeout_guard = Some(glib::timeout_add_local(Duration::from_millis(1000 / spin.value() as u64), {
                    let game = Arc::clone(&game);
                    let drawing_area = drawing_area.clone();
                    let is_running = Arc::clone(&is_running);
                    move || {
                        if is_running.load(Ordering::Acquire) {
                            if let Ok(mut game_guard) = game.lock() {
                                game_guard.evolve();
                            }
                            drawing_area.queue_draw();
                        }
                        glib::ControlFlow::Continue
                    }
                }));
            }
        }
    });
    
    window.present();
}