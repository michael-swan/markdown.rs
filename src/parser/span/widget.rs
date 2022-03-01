use parser::Span;
use parser::Span::Widget;
use regex::Regex;

pub fn parse_widget(text: &str) -> Option<(Span, usize)> {
    lazy_static! {
        static ref WIDGET_STR: &'static str = "^<(?P<uri>.+?)>";
        static ref WIDGET: Regex =
            Regex::new(&WIDGET_STR).unwrap();
    }

    if WIDGET.is_match(text) {
        let caps = WIDGET.captures(text).unwrap();
        let uri = caps.name("uri").unwrap().as_str();
        return Some((Widget(uri.to_owned()), uri.len() + 2));
    }
    None
}

#[cfg(test)]
mod test {
    use parser::span::parse_widget;
    use parser::Span::{Image, Link, Literal, RefLink, Text};

    #[test]
    fn finds_widget() {
        assert_eq!(
            parse_widget("<emu:/text/example>"),
            Some((
                Widget("emu:/text/example".to_owned()),
                19
            ))
        );
    }
}
