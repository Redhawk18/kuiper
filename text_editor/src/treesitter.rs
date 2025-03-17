use iced::advanced::text;
use std::{ops::Range, time::Duration};
use tree_house::{highlighter, tree_sitter::InputEdit};

const TIMEOUT: Duration = Duration::from_millis(250);

pub struct Loader {}

impl Loader {
    pub fn new() -> Self {
        Self {}
    }
}

impl tree_house::LanguageLoader for Loader {
    fn language_for_marker(
        &self,
        _marker: tree_house::InjectionLanguageMarker,
    ) -> Option<tree_house::Language> {
        let i = 0;
        Some(tree_house::Language(i as u32))
    }

    fn get_config(&self, _lang: tree_house::Language) -> Option<&tree_house::LanguageConfig> {
        None
        /*
         * Load the compiled parser with `tree_house::Grammar::new`.
         * Load query text from `highlights.scm`, `injections.scm` and `locals.scm` queries
         * with `tree_house::read_query`. And pass everything to `tree_house::LanguageConfig::new`.
         */
        // Some(config)
    }
}

pub struct Syntax(tree_house::Syntax);

impl Syntax {
    pub fn new() -> Self {
        let language = tree_house::Language(0 as u32);
        let loader = Loader::new();

        Self(
            tree_house::Syntax::new(
                "fn main() { let i = 5; }".into(),
                language,
                TIMEOUT,
                &loader,
            )
            .unwrap(),
        )
    }
}

pub struct Highlighter<'a> {
    current_line: usize,
    highlighter: highlighter::Highlighter<'a, 'a, Loader>,
    syntax: &'a Syntax,
}

#[derive(Clone)]
pub struct Settings<'a> {
    syntax: &'a Syntax,
    text: &'a str,
    loader: &'a Loader,
    range: Range<u32>,
}

/// We want to compare the content and the range of changes, comparing the syntax tree and the
/// loader are not needed.
impl PartialEq for Settings<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.text == other.text && self.range == other.range {
            true
        } else {
            false
        }
    }
}

impl<'a> text::Highlighter for Highlighter<'a>
where
    'a: 'static,
{
    type Settings = Settings<'a>;
    type Highlight = highlighter::Highlight;
    type Iterator<'b> = Box<dyn Iterator<Item = (Range<usize>, highlighter::Highlight)> + 'b>;

    fn new(settings: &Self::Settings) -> Self {
        Self {
            current_line: 0,
            highlighter: highlighter::Highlighter::new(
                &settings.syntax.0,
                settings.text.into(),
                settings.loader,
                settings.range.clone(),
            ),
            syntax: settings.syntax,
        }
    }

    /// Soft reset the syntax tree with new settings.
    fn update(&mut self, new_settings: &Self::Settings) {
        let edit = InputEdit {
            start_byte: todo!(),
            old_end_byte: todo!(),
            new_end_byte: todo!(),
            start_point: todo!(),
            old_end_point: todo!(),
            new_end_point: todo!(),
        };

        let _ = self.syntax.0.update(
            new_settings.text.into(),
            TIMEOUT,
            &[edit],
            new_settings.loader,
        );

        self.change_line(0);
    }

    fn change_line(&mut self, line: usize) {
        self.current_line = line;
    }

    fn highlight_line(&mut self, _line: &str) -> Self::Iterator<'_> {
        todo!()
    }

    fn current_line(&self) -> usize {
        self.current_line
    }
}
