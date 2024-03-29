use oxid_framework::prelude::*;

#[oxid_framework::main("Texture")]
async fn main() {
    let texture: Texture2D = load_texture("examples/ferris.png").await;

    loop {
        clear_background(RED);
        draw_texture(
            texture,
            screen_width() / 2. - texture.width() / 2.,
            screen_height() / 2. - texture.height() / 2.,
            WHITE,
        );
        next_frame().await
    }
}
