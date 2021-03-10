use fltk::*;
use mathical::{self, cli};

fn functioner(x: f64) -> f64 {
    x * x
}

fn drawer(){
    cli::graph::graph(functioner);
}

fn main() {
    let app = app::App::default();

    let mut win = window::Window::new(100, 100, 740, 580, "Mathical GUI");

    let mut framer = frame::Frame::new(0, 0, 400, 200, "");

    let mut graph = button::Button::new(160, 210, 80, 40, "Graph");
    
    graph.set_callback(move || framer.set_label("Ho"));
    graph.set_callback(move || cli::graph::graph(functioner));

    win.end();
    win.show();
    app.run().unwrap();
}
