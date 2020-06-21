use stray::raytracer::Stray; 

fn main()
{
    //NOTE(staneesh): mut because in the future
    // it will be possible to set params of rendering 
    // before drawing the scene.
    let mut stray = Stray::new();

    //TODO(staneesh): when dimenisons not equal image distorted!
    //stray.set_window_dimensions(1024, 512);

    //TODO(staneesh): rendering messed up after moving code
    // out of main.rs
    stray.render_scence();    

}
