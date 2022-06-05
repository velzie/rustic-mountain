use rustic_mountain_core::*;
use structures::*;
fn main() {
    let mut engine = Celeste::new("".into(), "".into(), "".into());
    // rustic_mountain_core::structures::
    // engine.objects.push(structures::Object {
    //     pos: Vector::new(0, 0),
    //     obj: &structures::Player {},
    // })
    engine.draw();
    engine.next_tick()
}
