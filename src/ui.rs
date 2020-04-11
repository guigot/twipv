extern crate serde_json;
extern crate cursive;


use use_mpv::callback_video;
use twitch::retrieve_videos;
use self::serde_json::{Value};
use cursive::align::HAlign;
use cursive::theme::{Color, PaletteColor};
use cursive::Cursive;
use cursive::traits::*;
use cursive::event::{Event};
use cursive::views::{Dialog,OnEventView,SelectView,DummyView,EditView,LinearLayout,NamedView,ViewRef};


pub fn construct_select_view(siv : &mut Cursive, last_videos : &str) {

    let mut select_view : ViewRef<SelectView> = siv.find_name::<SelectView>("select_view").unwrap();
    select_view.set_on_submit(callback_video);
    select_view.clear();

    let val: Value = serde_json::from_str(last_videos).unwrap();

    for _i in 0..10 {
        let mut plain_title : String = val["videos"][_i]["title"].as_str().unwrap().to_string();
        let size = 70;
        if plain_title.chars().count() > size {
            plain_title = plain_title.chars().take(size-3).collect();
            plain_title.push_str("...");
        }
        else {
            let fill = " ".repeat(size - plain_title.chars().count());
            plain_title.push_str(fill.as_str()); 
        }
                                                       
        let mut line_str = plain_title;
        line_str.push_str("  ");
        let game = val["videos"][_i]["game"].as_str().unwrap();
        line_str.push_str(game);

        select_view.add_item(line_str,
                            val["videos"][_i]["url"].as_str().unwrap().to_string());
    }
}

fn submit_streamer(siv : &mut Cursive, streamer : &str) {

    let last_videos = retrieve_videos(streamer);
    construct_select_view(siv, &last_videos);
}

fn construct_edit_view(siv : &mut Cursive) {

    let mut edit_view = siv.find_name::<EditView>("edit_view").unwrap();
    edit_view.set_on_submit(submit_streamer);

}

pub fn construct_ui(siv : &mut Cursive) {

    let last_videos = retrieve_videos("mistermv");
    let select_view : NamedView<SelectView> = SelectView::new()
        .h_align(HAlign::Left)
        .with_name("select_view");

    // TODO : Mettre espace pour "enter"
    let select_view = OnEventView::new(select_view)
        .on_event(Event::Char('k'), move |siv| {
            siv.call_on_name("select_view", |select_view : &mut SelectView| {
                select_view.select_up(1);
            });
        })
        .on_event(Event::Char('j'), move |siv| {
            siv.call_on_name("select_view", |select_view : &mut SelectView| {
                select_view.select_down(1);
            });
        });

    let edit_view = EditView::new()
        .with_name("edit_view");

    let mut theme = siv.current_theme().clone();
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    siv.set_theme(theme);

    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(select_view)
                .child(DummyView.fixed_height(1))
                .child(edit_view)
        )
        .title("Derniers streams MV")
    );

    construct_select_view(siv, &last_videos);
    construct_edit_view(siv);

    siv.add_global_callback('q', |s| s.quit());

}
