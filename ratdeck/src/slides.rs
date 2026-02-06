use ratatui::text::Text;

#[derive(Debug)]
pub enum Slide {
    Title(TitleSlide),
    Text(TextSlide),
    Image(ImageSlide),
}

#[derive(Debug)]
pub struct TitleSlide {
    pub title: &'static str,
}

#[derive(Debug)]
pub struct TextSlide {
    pub title: &'static str,
    pub text: &'static Text<'static>,
}

#[derive(Debug)]
pub struct ImageSlide {
    pub title: &'static str,
    pub image: &'static str,
    pub text: &'static Text<'static>,
}

include!(concat!(env!("OUT_DIR"), "/slides.rs"));
