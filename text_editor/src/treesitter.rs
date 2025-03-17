use iced::advanced::text;
use ouroboros::self_referencing;
use std::{ops::Range, sync::Arc};
use syntastica::Processor;
use syntastica_parsers::{self, Lang, LanguageSetImpl};

const LANGUAGE: Lang = Lang::Rust;

/// VOODOO WARNING, This is not ideal but it compiles this way.
///
/// This holds the data for the highlighter used by [`iced`] and by [`syntastica`],
/// Be careful this is held together with hope and prayers.
#[self_referencing]
pub struct Highlighter {
    current_line: Box<usize>,
    language_set: Box<LanguageSetImpl>,
    #[borrows(language_set)]
    #[covariant]
    processor: Processor<'this, LanguageSetImpl>,
}

type Highlight = Option<&'static str>;

/// The infomation that is passed into the highlighter for updates, or new highlights.
#[derive(Clone, PartialEq)]
pub struct Settings<'a> {
    text: &'a str,
}

impl text::Highlighter for Highlighter {
    type Settings = Settings<'static>;
    type Highlight = Highlight;
    type Iterator<'b> = Box<dyn Iterator<Item = (Range<usize>, &str)> + 'b>;

    fn new(_settings: &Self::Settings) -> Self {
        HighlighterBuilder {
            current_line: Box::new(0),
            language_set: Box::new(LanguageSetImpl::new()),
            processor_builder: |set| Processor::new(set),
        }
        .build()
    }

    fn update(&mut self, new_settings: &Self::Settings) {
        _ = self.with_processor_mut(|p| p.process(new_settings.text, LANGUAGE));
        self.change_line(0);
    }

    fn change_line(&mut self, line: usize) {
        self.with_current_line_mut(|l| **l = line);
    }

    fn highlight_line(&mut self, line: &'static str) -> Self::Iterator<'_> {
        // Yeah this can be dangerous. If this never got safer please fix it. Also this makes no
        // fucking sense, Iced does highlighting line by line, yet Syntastica does it by the whole
        // file. So this could be a huge bottleneck at most or at least is an annoyance to me.
        let highlights = self.with_processor_mut(|p| p.process(line, LANGUAGE).unwrap()[0].clone());

        let mut column = 0;
        let mut iter = Vec::new();
        for (text, _) in highlights {
            let start = column;
            let end = column + text.len();
            column += text.len();

            iter.push((start..end, text));
        }

        Box::new(iter.into_iter())
    }

    fn current_line(&self) -> usize {
        **self.borrow_current_line()
    }
}
