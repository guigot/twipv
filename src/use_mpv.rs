use cursive::Cursive;
use libmpv::{events::*, *};
use std::thread;

pub fn callback_video(_siv: &mut Cursive, url: &str) -> Result<()> {
    launch_video(url)
}

fn launch_video(video_path: &str) -> Result<()> {
    let videopath = String::from(video_path);

    thread::spawn(move || {
        let mpv = Mpv::with_initializer(|mpv_initializer| {
            mpv_initializer.set_property("osc", true)?;
            mpv_initializer.set_property("save-position-on-quit", true)?;
            let mpv_xdgdir = xdg::BaseDirectories::with_prefix("mpv").unwrap();
            let watchlater_dir = mpv_xdgdir.create_config_directory("watch_later").unwrap();
            let watchlater_dir = watchlater_dir.to_str().unwrap();
            mpv_initializer.set_property("watch-later-directory", watchlater_dir)?;
            mpv_initializer.set_property("input-default-bindings", true)?;
            mpv_initializer.set_property("input-vo-keyboard", true)?;
            Ok(())
        })
        .unwrap();

        mpv.playlist_load_files(&[(&videopath, FileState::AppendPlay, None)])
            .unwrap();
        let mut ev_ctx = mpv.create_event_context();

        loop {
            let ev = ev_ctx.wait_event(0.).unwrap_or(Err(Error::Null));
            match ev {
                Ok(Event::EndFile(_r)) => {
                    break;
                }

                Ok(_e) => continue,
                Err(_e) => continue,
            }
        }
    });
    Ok(())
}
