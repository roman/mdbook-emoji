use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CowStr, Event};
use regex::Regex;
use emojis;

pub struct EmojiPreprocessor;

impl EmojiPreprocessor {
    pub fn new() -> Self {
        EmojiPreprocessor
    }

    fn process_item(item: &mut BookItem) -> Result<(), Error> {
        if let BookItem::Chapter(ref mut ch) = item {
            ch.content = EmojiPreprocessor::process_content(&ch.content)?;
        }
        Ok(())
    }

    fn process_content(content: &str) -> Result<String, Error> {
        let parser = mdbook::utils::new_cmark_parser(content);
        let events = parser.map(EmojiPreprocessor::convert_event);
        let mut buffer = String::new();
        pulldown_cmark_to_cmark::cmark(events, &mut buffer, None)
            .map_err(|err| Error::new(err).context("Markdown serialization failed"))?;
        Ok(buffer)
    }

    fn convert_event(event: Event) -> Event {
        match event {
            Event::Text(ref text) => Event::Text(CowStr::from(convert_shortcodes_to_codepoints(text))),
            _ => event,
        }
    }
}

impl Preprocessor for EmojiPreprocessor {
    fn name(&self) -> &str {
        "emoji-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let mut err = None;
        book.for_each_mut(|item| {
            EmojiPreprocessor::process_item(item).unwrap_or_else(|e| {
                if err.is_none() {
                    err = Some(e);
                }
            })
        });
        err.map_or(Ok(book), Err)
    }
}

fn convert_shortcodes_to_codepoints(original_text: &str) -> String {
    let mut emoji_text: String = String::new();
    for line in original_text.lines() {
        let shortcode = Regex::new(r":[a-zA-Z_]*:").unwrap();
        if shortcode.is_match(&line) {
            let mut buffer: String = String::new();
            buffer.push_str(&line);
            for cap in shortcode.captures_iter(&line) {
                let shortcode = &cap[0].to_string();
                let emoji = emojis::lookup(&shortcode.replace(":",""));
                if emoji != None {
                    let emoji = emoji.unwrap().to_string();
                    let r = Regex::new(&shortcode).unwrap();
                    buffer = r.replace_all(&buffer, &emoji).to_string();
                }
            }
            emoji_text.push_str(&buffer);
        } else {
            emoji_text.push_str(&line);
        };
        emoji_text.push('\n');
    };
    return emoji_text;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn process_content() {
        let new_content =
            EmojiPreprocessor::process_content(":sparkling_heart:").unwrap();
        assert_eq!(new_content, "💖\n")
    }
}
