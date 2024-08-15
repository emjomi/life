use gtk::{prelude::*, glib, gio, Application, ApplicationWindow, DrawingArea};
use std::{sync::{atomic::{Ordering, AtomicBool}, Arc, Mutex}, time::Duration};
use super::game::{Game, Cell::*};

pub fn build_ui(app: &Application) {    
    let game = Arc::new(Mutex::new(Game::builder().random_grid(50).build()));
    let drawing_area = DrawingArea::new();
    let is_running = Arc::new(AtomicBool::new(true));
    
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
    
    let toggle_running_action = gio::SimpleAction::new("toggle_running", None);
    let randomize_grid_action = gio::SimpleAction::new("randomize_grid", None);
    let clear_grid_action = gio::SimpleAction::new("clear_grid", None);
    let evolve_action = gio::SimpleAction::new("evolve", None);

    toggle_running_action.connect_activate({
        let is_run = Arc::clone(&is_running);
        move |_, _| {
            is_run.fetch_xor(true, Ordering::AcqRel);
        }
    });
    randomize_grid_action.connect_activate({
        let game = Arc::clone(&game);
        let drawing_area = drawing_area.clone();
        move |_, _| {
            if let Ok(mut game_guard) = game.lock() {
                game_guard.randomize_grid();
                drawing_area.queue_draw();
            }
        }
    });
    clear_grid_action.connect_activate({
        let game = Arc::clone(&game);
        let drawing_area = drawing_area.clone();
        move |_, _| {
            if let Ok(mut game_guard) = game.lock() {
                game_guard.clear_grid();
                drawing_area.queue_draw();
            }
        }
    });
    evolve_action.connect_activate({
        let is_running = Arc::clone(&is_running);
        let game = Arc::clone(&game);
        let drawing_area = drawing_area.clone();
        move |_, _| {
            if !is_running.load(Ordering::Acquire) {
                if let Ok(mut game_guard) = game.lock() {
                    game_guard.evolve();
                    drawing_area.queue_draw();
                }
            }
        }
    });
    
    app.add_action(&toggle_running_action);
    app.add_action(&randomize_grid_action);
    app.add_action(&clear_grid_action);
    app.add_action(&evolve_action);

    app.set_accels_for_action("app.toggle_running", &["space", "k"]);
    app.set_accels_for_action("app.randomize_grid", &["<Ctrl>R"]);
    app.set_accels_for_action("app.clear_grid", &["<Ctrl>E"]);
    app.set_accels_for_action("app.evolve", &["Right", "l"]);
    
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Life")
        .default_width(400)
        .default_height(400)
        .child(&drawing_area)
        .build();
    
    glib::timeout_add_local(Duration::from_millis(1000 / 30), 
        move || {
            if is_running.load(Ordering::Acquire) {
                drawing_area.queue_draw();
                if let Ok(mut game_guard) = game.lock() {
                    game_guard.evolve();
                }
            }
            glib::ControlFlow::Continue
        }
    );
    
    window.present();
}