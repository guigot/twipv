extern crate serde_json;
extern crate cursive;


use use_mpv::callback_video;
use twitch::retrieve_videos;
use self::serde_json::{Value};
use cursive::align::HAlign;
use cursive::theme::{Color, PaletteColor};
use cursive::Cursive;
use cursive::traits::*;
use cursive::event::EventResult;
use cursive::views::{Dialog,OnEventView,SelectView,DummyView,EditView,LinearLayout};


pub fn construct_select_view(last_videos : &str) -> SelectView {

    let mut select_view = SelectView::new()
        .h_align(HAlign::Left);
    select_view.set_on_submit(callback_video);

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

    select_view
}

// fn construct_edit_view() -> EditView {


// }

pub fn construct_ui(siv : &mut Cursive) {

    let last_videos = retrieve_videos("mistermv");
    let select_view = construct_select_view(&last_videos);

    let select_view = OnEventView::new(select_view)
        .on_pre_event_inner('k', |s, _| {
            s.select_up(1);
            Some(EventResult::Consumed(None))
        }).on_pre_event_inner('j', |s, _| {
            s.select_down(1);
            Some(EventResult::Consumed(None))
        });


    let edit_view = EditView::new();

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

}
