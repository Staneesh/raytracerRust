use stray::raytracer::Stray;

fn main() {
    let mut stray = Stray::new();

    stray.set_window_dimensions(350, 250);
    stray.set_background((0.2, 0.2, 0.2));
    stray.set_number_of_threads(4);

    stray.add_material((1.0, 0.5, 0.1), 0.7, 0).unwrap();
    stray.add_emit_material((0.8, 0.5, 0.5), 1).unwrap();
    stray.add_emit_material((0.0, 0.0, 1.0), 2).unwrap();

    stray.add_sphere((0.0, -1.0, -5.0), 1.0, 0).unwrap();
    stray.add_sphere((0.0, 1.0, -5.0), 1.0, 1).unwrap();
    stray.add_sphere((1.0, 0.0, -4.0), 0.5, 2).unwrap();

    stray.render_scence();
}
