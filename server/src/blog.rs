use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Options, Parser, Tag};

const CUT_VALUE: &str = "%%CUT%%\n";

#[derive(Debug, Serialize, Deserialize)]
/// Blog post without HTML
pub struct BlogPostNoHTML {
    pub title: String,
    pub md_contents: String,
    pub date: NaiveDate,
    pub url: String,
}

impl BlogPostNoHTML {
    fn render_md(md: &str) -> String {
        let opts = Options::empty();
        let mut s = String::with_capacity(&md.len() * 3 / 2);
        let p = Parser::new_ext(&md, opts);

        // Setup for syntect to highlight (specifically) Rust code
        let ss = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let theme = &ts.themes["base16-ocean.dark"];

        // We'll build a new vector of events since we can only consume the parser once
        let mut new_p = Vec::new();
        // As we go along, we'll want to highlight code in bundles, not lines
        let mut to_highlight = String::new();
        // And track a little bit of state
        let mut in_code_block: Option<CowStr> = None;

        for event in p {
            match event {
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                    // In actual use you'd probably want to keep track of what language this code is
                    in_code_block = Some(lang);
                }
                Event::End(Tag::CodeBlock(_)) => {
                    if let Some(code_block_kind) = in_code_block {
                        // Format the whole multi-line code block as HTML all at once
                        let syntax = ss
                            .find_syntax_by_token(code_block_kind.as_ref())
                            .unwrap_or_else(|| ss.find_syntax_plain_text());
                        let html = highlighted_html_for_string(&to_highlight, &ss, &syntax, &theme);
                        // And put it into the vector
                        new_p.push(Event::Html(CowStr::Boxed(html.into_boxed_str())));
                        to_highlight = String::new();
                        in_code_block = None;
                    }
                }
                Event::Text(t) => {
                    if in_code_block.is_some() {
                        // If we're in a code block, build up the string of text
                        to_highlight.push_str(&t);
                    } else {
                        new_p.push(Event::Text(t))
                    }
                }
                e => {
                    new_p.push(e);
                }
            }
        }

        // Now we send this new vector of events off to be transformed into HTML
        html::push_html(&mut s, new_p.into_iter());
        s
    }

    pub fn render(self) -> BlogPostHTML {
        return BlogPostHTML {
            title: self.title,
            date: self.date,
            url: self.url,
            html_preview: Self::render_md(self.md_contents.split(CUT_VALUE).next().unwrap()),
            html_contents: Self::render_md(&self.md_contents.replacen(CUT_VALUE, "", 1)),
            md_contents: self.md_contents.clone(),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// Blog post for preview
pub struct BlogPostPreview {
    pub html_preview: String,
    pub title: String,
    pub date: NaiveDate,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Blog post for display
pub struct BlogPostDisplay {
    pub html_contents: String,
    pub title: String,
    pub date: NaiveDate,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Blog post with full HTML and preview
/// Used in DB
pub struct BlogPostHTML {
    pub html_contents: String,
    pub html_preview: String,

    pub title: String,
    pub md_contents: String,
    pub date: NaiveDate,
    pub url: String,
}
