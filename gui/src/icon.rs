use iced_aw::core::icons::nerd::Nerd;
use std::path::Path;

pub use iced_aw::core::icons::nerd::icon_to_char;

/// Maps the file name to [`pattern_code::Languages`]'s and matches and returns the icon's char
/// value
pub fn path_to_char(path: &Path) -> char {
    icon_to_char(match pattern_code::path_to_language(path) {
        pattern_code::Language::Assembly => Nerd::FileBinary,
        pattern_code::Language::Bash => Nerd::Bash,
        pattern_code::Language::Batch => Nerd::Powershell,
        pattern_code::Language::C => Nerd::LanguageC,
        pattern_code::Language::Cargo => Nerd::ICustomToml,
        pattern_code::Language::Cargolock => Nerd::Lock,
        pattern_code::Language::CMake => Nerd::Gear,
        pattern_code::Language::CPP => Nerd::LanguageCpp,
        pattern_code::Language::CSS => Nerd::LanguageCssthree,
        pattern_code::Language::DockerCompose => Nerd::Docker,
        pattern_code::Language::Dockerfile => Nerd::Docker,
        pattern_code::Language::DockerIgnore => Nerd::Docker,
        pattern_code::Language::Elixir => Nerd::ISetiElixirScript,
        pattern_code::Language::Elm => Nerd::ICustomElm,
        pattern_code::Language::Env => Nerd::Gear,
        pattern_code::Language::Erlang => Nerd::File,
        pattern_code::Language::GitIgnore => Nerd::Git,
        pattern_code::Language::Go => Nerd::LanguageGo,
        pattern_code::Language::H => Nerd::LanguageC,
        pattern_code::Language::Haskell => Nerd::LanguageHaskell,
        pattern_code::Language::HPP => Nerd::LanguageCpp,
        pattern_code::Language::HTML => Nerd::LanguageHtmlfive,
        pattern_code::Language::Java => Nerd::LanguageJava,
        pattern_code::Language::JavaScript => Nerd::LanguageJavascript,
        pattern_code::Language::Json => Nerd::Json,
        pattern_code::Language::Jupyter => Nerd::LanguagePython,
        pattern_code::Language::Kotlin => Nerd::LanguageKotlin,
        pattern_code::Language::Lisp => Nerd::ICustomCommonLisp,
        pattern_code::Language::Lua => Nerd::LanguageLua,
        pattern_code::Language::Makefile => Nerd::ISetiMakefile,
        pattern_code::Language::Markdown => Nerd::MarkdownOne,
        pattern_code::Language::Nix => Nerd::Nix,
        pattern_code::Language::OCaml => Nerd::ISetiOcaml,
        pattern_code::Language::Perl => Nerd::ISetiPerl,
        pattern_code::Language::PHP => Nerd::LanguagePhp,
        pattern_code::Language::PowerShell => Nerd::TerminalPowershell,
        pattern_code::Language::Python => Nerd::LanguagePython,
        pattern_code::Language::R => Nerd::LanguageR,
        pattern_code::Language::Racket => Nerd::Code,
        pattern_code::Language::ReadMe => Nerd::MarkdownOne,
        pattern_code::Language::Ruby => Nerd::LanguageRuby,
        pattern_code::Language::Rust => Nerd::LanguageRust,
        pattern_code::Language::Shell => Nerd::Terminal,
        pattern_code::Language::SQL => Nerd::Database,
        pattern_code::Language::Svelte => Nerd::ISetiSvelte,
        pattern_code::Language::SVG => Nerd::Svg,
        pattern_code::Language::Swift => Nerd::LanguageSwift,
        pattern_code::Language::Text => Nerd::Text,
        pattern_code::Language::Toml => Nerd::ICustomToml,
        pattern_code::Language::Typescript => Nerd::LanguageTypescript,
        pattern_code::Language::Vue => Nerd::Vuejs,
        pattern_code::Language::XAML => Nerd::LanguageXaml,
        pattern_code::Language::XML => Nerd::Xml,
        pattern_code::Language::Yaml => Nerd::Code,
        pattern_code::Language::Zig => Nerd::ISetiZig,
        pattern_code::Language::Zsh => Nerd::TerminalBash,
        pattern_code::Language::Unknown => Nerd::File,
    })
}

pub fn file() -> char {
    icon_to_char(Nerd::File)
}
