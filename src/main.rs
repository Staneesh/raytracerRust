use stray::raytracer::Stray; 

fn main()
{
    //NOTE(staneesh): mut because in the future
    // it will be possible to set params of rendering 
    // before drawing the scene.
    let mut stray = Stray::new();

    stray.add_material(1.0, 0.0, 0.0, 0.5, 0);
    stray.add_sphere(0.0, -1.0, -5.0, 2.0);
    stray.add_sphere(0.0, 1.0, -5.0, 2.0);

    //TODO(staneesh): when dimenisons not equal image distorted!
    //stray.set_window_dimensions(1024, 512);

    stray.render_scence();    

}
