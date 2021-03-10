use fltk::*;
use mathical::{self, cli};

fn functioner(x: f64) -> f64 {
    x * x
}

fn main() {
    let app = app::App::default();

    let mut win = window::Window::new(100, 100, 740, 580, "Mathical GUI");

    let mut graph = button::Button::new(160, 210, 80, 40, "Graph");

    graph.set_callback(move || cli::graph::graph(functioner));

    win.end();
    win.show();
    app.run().unwrap();
}
