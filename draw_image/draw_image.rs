use nannou::prelude::*;

struct Model {
    texture: wgpu::Texture,
}

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    // Create a new window
    app.new_window().size(512, 512).view(view).build().unwrap();

    // Load the image from disk and upload it to a GPU texture
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("nature_1.jpg");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();

    Model { texture }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let win = app.window_rect();
    let r = Rect::from_w_h(100.0, 100.0).top_left_of(win);

    let draw = app.draw();
    draw.texture(&model.texture).xy(r.xy()).wh(r.wh());

    draw.to_frame(app, &frame).unwrap();
}
