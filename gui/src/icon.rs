use iced_fonts::Nerd;
use std::path::Path;

use iced_fonts::nerd::icon_to_char;
use pattern_code::Language;

/// Maps the file name to [`Languages`]'s and matches and returns the icon's char
/// value
pub fn path_to_char(path: &Path) -> char {
    icon_to_char(match Language::from(path) {
        Language::Assembly => Nerd::FileBinary,
        Language::Bash => Nerd::Bash,
        Language::Batch => Nerd::Powershell,
        Language::C => Nerd::LanguageC,
        Language::Cargo => Nerd::ICustomToml,
        Language::Cargolock => Nerd::Lock,
        Language::CMake => Nerd::Gear,
        Language::CPP => Nerd::LanguageCpp,
        Language::CSS => Nerd::LanguageCssthree,
        Language::DockerCompose => Nerd::Docker,
        Language::Dockerfile => Nerd::Docker,
        Language::DockerIgnore => Nerd::Docker,
        Language::Elixir => Nerd::ISetiElixirScript,
        Language::Elm => Nerd::ICustomElm,
        Language::Env => Nerd::Gear,
        Language::Erlang => Nerd::File,
        Language::GitIgnore => Nerd::Git,
        Language::Go => Nerd::LanguageGo,
        Language::H => Nerd::LanguageC,
        Language::Haskell => Nerd::LanguageHaskell,
        Language::HPP => Nerd::LanguageCpp,
        Language::HTML => Nerd::LanguageHtmlfive,
        Language::Java => Nerd::LanguageJava,
        Language::JavaScript => Nerd::LanguageJavascript,
        Language::Json => Nerd::Json,
        Language::Jupyter => Nerd::LanguagePython,
        Language::Kotlin => Nerd::LanguageKotlin,
        Language::Lisp => Nerd::ICustomCommonLisp,
        Language::Lua => Nerd::LanguageLua,
        Language::Makefile => Nerd::ISetiMakefile,
        Language::Markdown => Nerd::MarkdownOne,
        Language::Nix => Nerd::Nix,
        Language::OCaml => Nerd::ISetiOcaml,
        Language::Perl => Nerd::ISetiPerl,
        Language::PHP => Nerd::LanguagePhp,
        Language::PowerShell => Nerd::TerminalPowershell,
        Language::Python => Nerd::LanguagePython,
        Language::R => Nerd::LanguageR,
        Language::Racket => Nerd::Code,
        Language::ReadMe => Nerd::MarkdownOne,
        Language::Ruby => Nerd::LanguageRuby,
        Language::Rust => Nerd::LanguageRust,
        Language::Shell => Nerd::Terminal,
        Language::SQL => Nerd::Database,
        Language::Svelte => Nerd::ISetiSvelte,
        Language::SVG => Nerd::Svg,
        Language::Swift => Nerd::LanguageSwift,
        Language::Text => Nerd::Text,
        Language::Toml => Nerd::ICustomToml,
        Language::Typescript => Nerd::LanguageTypescript,
        Language::Vue => Nerd::Vuejs,
        Language::XAML => Nerd::LanguageXaml,
        Language::XML => Nerd::Xml,
        Language::Yaml => Nerd::Code,
        Language::Zig => Nerd::ISetiZig,
        Language::Zsh => Nerd::TerminalBash,
        Language::Unknown => Nerd::File,
    })
}

pub fn file() -> char {
    icon_to_char(Nerd::File)
}
