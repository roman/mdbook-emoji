use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CowStr, Event, Tag};
use regex::Regex;
use emojis;

pub struct EmojiPreprocessor {
    convert_text: bool,
}

impl EmojiPreprocessor {
    pub fn new() -> Self {
        EmojiPreprocessor {
            convert_text: true,
        }
    }

    fn process_item(item: &mut BookItem) -> Result<(), Error> {
        if let BookItem::Chapter(ref mut ch) = item {
            ch.content = EmojiPreprocessor::process_content(&ch.content)?;
        }
        Ok(())
    }

    fn process_content(content: &str) -> Result<String, Error> {
        let parser = mdbook::utils::new_cmark_parser(content);
        let mut preprocess = EmojiPreprocessor::new();
        let events = parser.map(|event| preprocess.convert_event(event));
        let mut buffer = String::with_capacity(content.len());
        pulldown_cmark_to_cmark::cmark(events, &mut buffer, None)
            .map_err(|err| Error::new(err).context("Markdown serialization failed"))?;
        Ok(buffer)
    }

    fn convert_event<'a>(&mut self, event: Event<'a>) -> Event<'a> {
        match event {
            Event::Start(Tag::CodeBlock(_)) => {
                self.convert_text = false;
                event
            }
            Event::End(Tag::CodeBlock(_)) => {
                self.convert_text = true;
                event
            }
            Event::Text(ref text) if self.convert_text => {
                Event::Text(CowStr::from(convert_shortcodes_to_codepoints(text)))
            }
            _ => event,
        }
    }

    fn process_capture(shortcode: String, buffer: &str) -> Result<String, Error> {
        let emoji = emojis::lookup(&shortcode.replace(":",""));
        if emoji != None {
            return Ok(Regex::new(&shortcode)
                        .unwrap()
                        .replace_all(
                            &buffer,
                            &emoji
                                .unwrap()
                                .to_string()
                        ).into_owned());
        } else {
            return Ok(buffer.to_owned())
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
    let pattern = Regex::new(r":[a-zA-Z_]*:").unwrap();
    let mut buffer: String = String::new();
    let mut patterns: Vec<String> = Vec::new();
    buffer.push_str(&original_text);
    if pattern.is_match(&original_text) {
        for capture in pattern.captures_iter(&original_text) {
            patterns.push(capture[0].to_string())
        }
        patterns.sort();
        patterns.dedup();
        for shortcode in patterns {
            buffer = EmojiPreprocessor::process_capture(shortcode, &buffer).unwrap();
        }
    }
    return buffer;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn process_content() {
        let new_content =
            EmojiPreprocessor::process_content(":sparkling_heart: `:smile:` :smile:\n:sparkling_heart: :bowtie:\n```\n:smile:\n````").unwrap();
        assert_eq!(new_content, "💖 `:smile:` 😄\n💖 :bowtie:\n\n````\n:smile:\n````")
    }
}
