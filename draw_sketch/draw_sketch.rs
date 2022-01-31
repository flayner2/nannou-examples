use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // Get a canvas to draw on
    let draw = app.draw();

    // Set background color (to blue)
    draw.background().color(BLUE);

    // Put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
