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
fn view(app: &App, _model: &Model, frame: Frame) {
    let ellipse_size = 100.0;
    let pad = ellipse_size / 2.0;

    // Create a sketch to draw on
    let draw = app.draw();

    // Feed the elapsed application time to a sine function
    let sine = app.time.sin();
    // Do the same but at a slower rate
    let slower_sine = (app.time / 2.0).sin();

    // Get the window Rect to know the window boundaries
    let boundary = app.window_rect();

    // Mapping the sine wave values to point positions at the screen
    let x = map_range(
        sine,
        -1.0,
        1.0,
        boundary.left() + pad,
        boundary.right() - pad,
    );
    let y = map_range(
        slower_sine,
        -1.0,
        1.0,
        boundary.bottom() + pad,
        boundary.top() - pad,
    );

    // Clear the background to purple
    draw.background().color(PLUM);

    // Draw a blue ellipse with a radius
    draw.ellipse()
        .color(STEELBLUE)
        .x_y(x, y)
        .w_h(ellipse_size, ellipse_size);

    draw.to_frame(app, &frame).unwrap();
}
