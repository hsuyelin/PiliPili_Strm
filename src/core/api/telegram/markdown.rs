use std::fmt;

#[derive(Debug, Default)]
pub struct MarkdownV2Builder {

    text: String,
}

impl MarkdownV2Builder {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text.push_str(&Self::escape(text));
        self
    }

    pub fn bold(self, text: &str) -> Self {
        self.text(&format!("*{}*", Self::escape(text)))
    }

    pub fn italic(self, text: &str) -> Self {
        self.text(&format!("_{}_", Self::escape(text)))
    }

    pub fn link(self, text: &str, url: &str) -> Self {
        self.text(&format!("[{}]({})", Self::escape(text), Self::escape(url)))
    }

    pub fn build(self) -> String {
        self.text
    }

    fn escape(text: &str) -> String {
        text.replace('*', r"\*")
            .replace('_', r"\_")
            .replace('[', r"\[")
            .replace(']', r"\]")
            .replace('(', r"\(")
            .replace(')', r"\)")
            .replace('~', r"\~")
            .replace('`', r"\`")
            .replace('>', r"\>")
            .replace('#', r"\#")
            .replace('+', r"\+")
            .replace('-', r"\-")
            .replace('=', r"\=")
            .replace('|', r"\|")
            .replace('{', r"\{")
            .replace('}', r"\}")
            .replace('.', r"\.")
            .replace('!', r"\!")
    }
}

impl fmt::Display for MarkdownV2Builder {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}