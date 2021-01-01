use oxid::*;

struct Stage;
impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        ctx.clear(Some((0., 1., 0., 1.)), None, None);
    }
}

fn main() {
    oxid::start(conf::Conf::default(), |ctx| UserData::owning(Stage, ctx));
}
