use adw::{gio, glib, prelude::*, Application, ApplicationWindow, HeaderBar, ToolbarView};
use gtk::{DrawingArea, MenuButton, ShortcutsGroup, ShortcutsSection, ShortcutsShortcut, ShortcutsWindow};
use std::{sync::{atomic::{Ordering, AtomicBool}, Arc, Mutex}, time::Duration};
use super::game::{Game, Cell::*};

pub fn build_ui(app: &Application) {    
    let game = Arc::new(Mutex::new(Game::builder().random_grid(30).build()));
    let is_running = Arc::new(AtomicBool::new(true));
    let drawing_area = DrawingArea::new();
    drawing_area.set_cursor_from_name(match !is_running.load(Ordering::Acquire) {
        false => Some("none"),
        true => None
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
                            context.set_source_rgb(0., 0., 0.);
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
                false => Some("none"),
                true => None
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

    app.set_accels_for_action("app.toggle_running", &["space"]);
    app.set_accels_for_action("app.randomize_grid", &["<Ctrl>r"]);
    app.set_accels_for_action("app.clear_grid", &["<Ctrl>e"]);
    app.set_accels_for_action("app.evolve", &["Right"]);
    app.set_accels_for_action("app.show_help_overlay", &["<Ctrl>question"]);
    
    let shortcuts_window = ShortcutsWindow::builder().build();
    let shortcuts_section = ShortcutsSection::builder().build();
    let shortcuts_group = ShortcutsGroup::builder().title("General").build();
    
    shortcuts_window.add_section(&shortcuts_section);
    shortcuts_section.add_group(&shortcuts_group);
    
    shortcuts_group.add_shortcut(&ShortcutsShortcut::builder().title("Toggle Running").action_name("app.toggle_running").accelerator("space").build());
    shortcuts_group.add_shortcut(&ShortcutsShortcut::builder().title("Evolve Step").action_name("app.evolve").accelerator("Right").build());
    shortcuts_group.add_shortcut(&ShortcutsShortcut::builder().title("Randomize Grid").action_name("app.randomize_grid").accelerator("<Ctrl>r").build());
    shortcuts_group.add_shortcut(&ShortcutsShortcut::builder().title("Clear Grid").action_name("app.clear_grid").accelerator("<Ctrl>e").build());
    shortcuts_group.add_shortcut(&ShortcutsShortcut::builder().title("Show shortcuts").action_name("app.show_help_overlay").accelerator("<Ctrl>question").build());

    show_help_overlay_action.connect_activate({
        move |_, _| {
            shortcuts_window.present();
        }
    });
    
    let menu = gio::Menu::new();
    menu.append(Some("_Toggle Running"), Some("app.toggle_running"));
    menu.append(Some("_Randomize Grid"), Some("app.randomize_grid"));
    menu.append(Some("_Clear Grid"), Some("app.clear_grid"));
    menu.append(Some("_Evolve Step"), Some("app.evolve"));
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
    
    glib::timeout_add_local(Duration::from_millis(1000 / 30), 
        move || {
            if is_running.load(Ordering::Acquire) {
                if let Ok(mut game_guard) = game.lock() {
                    game_guard.evolve();
                }
                drawing_area.queue_draw();
            }
            glib::ControlFlow::Continue
        }
    );
    
    window.present();
}