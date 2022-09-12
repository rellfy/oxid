use rwasm;
use oxid::framework::prelude::*;

#[rwasm::main]
async fn main() {
    oxid::framework::Window::new("basic_shapes", render());
}

#[no_mangle]
fn load_scripts() {
    rwasm::send_bytes("load_script", oxid::js().as_bytes());
}

async fn render() {
    loop {
        clear_background(BLACK);
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 30.0, WHITE);
        next_frame().await
    }
}
