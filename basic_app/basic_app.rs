use nannou::prelude::*;

struct Model {} // State of the application

fn main() {
    nannou::app(model) // Start building the app and specify model
        .event(event) // Specify that we want to handle app events with `event()`
        .simple_window(view) // Request a simple window to which we'll draw with `view()`
        .run(); // Run the app
}

// Runs once at the beginning of the program (e.g. `setup()`)
// Creates a new instance of the model
fn model(_app: &App) -> Model {
    Model {}
}

// Runs every time an app event happens (e.g. key and mouse presses)
fn event(_app: &App, _model: &mut Model, _event: Event) {}

// Presents the state of the model to the window by drawing to its Frame and returning the same
// frame at the end. Basically`draw()`
fn view(_app: &App, _model: &Model, _frame: Frame) {}
