use gtk::{glib, prelude::*, Application, ApplicationWindow, DrawingArea};
use std::{time::Duration, sync::{Arc, Mutex}};
use super::game::{Game, Cell::*};

pub fn build_ui(app: &Application) {    
    let game = Arc::new(Mutex::new(Game::builder().random_grid(50).build()));
    let drawing_area = DrawingArea::new();
    
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
    
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Life")
        .default_width(400)
        .default_height(400)
        .child(&drawing_area)
        .build();
    
    glib::timeout_add_local(Duration::from_millis(1000 / 30), 
        move || {
            drawing_area.queue_draw();
            if let Ok(mut game_guard) = game.lock() {
                game_guard.evolve();
            }
            glib::ControlFlow::Continue
        }
    );
    
    window.present();
}