

use zoon::{*, named_color::*, web_sys::HtmlDivElement};
use crate::app;

pub fn page() -> impl Element {
    Column::new()
        .s(Background::new().color(PINK_3))
        .item(picture())
        .item(picture2())
        
        }

fn picture() -> impl Element {
    Image::new()
        .url("https://www.catastrophicreations.com/wp-content/uploads/2021/02/IMG_7465-2glance.jpg%22")
        .description("A cat")
        .s(Width::new(600))
        .s(Padding::top(Default::default(), 50))
}

fn picture2() -> impl Element {
    Image::new()
    .url("https://assets2.rockpapershotgun.com/elden-ring-limgrave-vista.jpg/BROK/resize/1920x1920%3E/format/jpg/quality/80/elden-ring-limgrave-vista.jpg")
    .description("Fun")
    .s(Width::new(600))
    .s(Padding::top(Default::default(), 50))
}
