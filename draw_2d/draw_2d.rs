use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // Get the canvas to draw
    let draw = app.draw();

    // Clear the blackground to purple
    draw.background().color(PLUM);

    // Draw a shape
    // draw.rect().color(STEELBLUE).w(300.0).h(200.0);

    // Defining points to draw a quad
    //let point1 = pt2(-10.0, -20.0);
    //let point2 = pt2(10.0, -30.0);
    //let point3 = pt2(15.0, 40.0);
    //let point4 = pt2(-20.0, 35.0);

    // Drawing a quad (should be useful for meshes)
    //draw.quad()
    //.color(STEELBLUE)
    //.w(300.0)
    //.h(200.0)
    //.points(point1, point2, point3, point4);

    // Could also draw a triangle (even better for meshes)
    //draw.tri()
    //.color(STEELBLUE)
    //.w(300.0)
    //.h(200.0)
    //.points(point1, point2, point3);

    // Defining points to draw a line
    //let start_point = pt2(-30.0, -20.0);
    //let end_point = pt2(40.0, 40.0);

    // Drawing a line
    //draw.line()
    //.start(start_point)
    //.end(end_point)
    //.weight(4.0)
    //.color(STEELBLUE);

    // Defining points to draw a sine wave-shaped polyline
    //let points = (0..50).map(|i| {
    //let x = i as f32 - 25.0; // Subtract 25 to center the sine wave
    //let point = pt2(x, x.sin()) * 20.0; // Scale by 20
    //(point, STEELBLUE)
    //});

    // Draw the sine wave
    // draw.polyline().weight(3.0).points_colored(points);

    // Can also generate a circle with polylines
    // Store the radius
    //let radius = 150.0;
    // Map over an aray of integers from 0 to 360 to represent the degrees in a circle
    //let points = (0..=360).map(|i| {
    // Convert each degree to radians
    //let radian = deg_to_rad(i as f32);
    // Get the sine of the radian to find the x coordinate of this point in the circle and
    // multiply it by the radius
    //let x = radian.sin() * radius;
    // Do the same with cosine to find the y coordinate
    //let y = radian.cos() * radius;
    // Construct and return a point object with a color
    //(pt2(x, y), STEELBLUE)
    //});

    // Draw the circle as a polyline
    //draw.polyline().weight(3.0).points_colored(points);

    // Can also draw an outline of a different shape by using a step
    let radius = 150.0;
    let points = (0..=360).step_by(45).map(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * radius;
        let y = radian.cos() * radius;
        (pt2(x, y), STEELBLUE)
    });

    // Draw the outline as a polyline
    //draw.polyline().weight(3.0).points_colored(points);
    // Almost the same code to draw a filled polygon
    draw.polygon().points_colored(points);

    // Draw to the window frame
    draw.to_frame(app, &frame).unwrap();
}
