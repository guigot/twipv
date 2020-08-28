// extern crate mpv; 
extern crate cursive;
extern crate libmpv; 


use std::path::Path;
use cursive::Cursive;

use use_mpv::libmpv::{events::*, *};

pub fn callback_video(_siv: &mut Cursive, url: &str) -> Result<()> {
    launch_video(Path::new(url))
}

fn launch_video(video_path: &Path) -> Result<()> {

    let mpv = Mpv::with_initializer(|mpv_initializer| {
        mpv_initializer.set_property("osc", true)?;
        mpv_initializer.set_property("save-position-on-quit", true)?;
        // TODO : chemin relatif
        mpv_initializer.set_property("watch-later-directory", "/home/exosta/.config/mpv/watch_later")?;
        mpv_initializer.set_property("input-default-bindings",true)?;
        mpv_initializer.set_property("input-vo-keyboard",true)?;
        Ok(())
    })
    .unwrap();


    let mut ev_ctx = mpv.create_event_context();
    
    let video_path = video_path.to_str().expect("Expected a string for Path, got None");

    crossbeam::scope(|scope| {
        scope.spawn(|_| {
            mpv.playlist_load_files(&[(&video_path, FileState::AppendPlay, None)])
                .unwrap();

        });
        scope.spawn(move |_| loop {
            let ev = ev_ctx.wait_event(0.).unwrap_or(Err(Error::Null));

            match ev {
                Ok(Event::EndFile(_r)) => {
                    break;
                }

                Ok(_e) => continue,
                Err(_e) => continue,
            }
        });
    })
    .unwrap();
    Ok(())

}
