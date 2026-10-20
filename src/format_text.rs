use std::ops::Range;
use std::path::Path;

use shuck_formatter::FormattedSource;
use shuck_formatter::IndentStyle;
use shuck_formatter::ShellDialect;
use shuck_formatter::ShellFormatOptions;
use shuck_formatter::TextRange;
use shuck_formatter::TextSize;

use crate::configuration::Configuration;
use crate::configuration::Dialect;

/// Error that occurred while formatting shell script text.
#[derive(Debug, thiserror::Error)]
pub enum FormatError {
  /// The text could not be parsed or formatted by shuck-formatter.
  #[error("Formatting failed: {0}")]
  Shuck(#[from] shuck_formatter::FormatError),
  /// The range does not fit in the offsets shuck-formatter supports.
  #[error(transparent)]
  RangeTooLarge(#[from] std::num::TryFromIntError),
}

pub fn format_text(file_path: &Path, input_text: &str, config: &Configuration) -> Result<Option<String>, FormatError> {
  let options = build_options(config, input_text, file_path);
  match shuck_formatter::format_source(input_text, Some(file_path), &options)? {
    FormattedSource::Formatted(formatted) => Ok(Some(formatted)),
    FormattedSource::Unchanged => Ok(None),
  }
}

/// Formats only the part of the text within the provided byte range.
///
/// shuck-formatter widens the range to the whole lines of the statements it touches and the text
/// outside of those is left as it was.
pub fn format_text_range(
  file_path: &Path,
  input_text: &str,
  range: Range<usize>,
  config: &Configuration,
) -> Result<Option<String>, FormatError> {
  let options = build_options(config, input_text, file_path);
  let range = clamp_to_char_boundaries(input_text, range);
  let text_range = TextRange::new(
    TextSize::new(u32::try_from(range.start)?),
    TextSize::new(u32::try_from(range.end)?),
  );
  let Some(result) = shuck_formatter::format_range(input_text, text_range, Some(file_path), &options)? else {
    return Ok(None);
  };
  let FormattedSource::Formatted(content) = result.formatted else {
    return Ok(None);
  };

  let range_start = usize::from(result.range.start());
  let range_end = usize::from(result.range.end());
  if range_start > range_end || range_end > input_text.len() {
    return Ok(None);
  }

  let mut formatted = String::with_capacity(input_text.len() - (range_end - range_start) + content.len());
  formatted.push_str(&input_text[..range_start]);
  formatted.push_str(&content);
  formatted.push_str(&input_text[range_end..]);
  Ok(if formatted == input_text { None } else { Some(formatted) })
}

fn build_options(config: &Configuration, input_text: &str, file_path: &Path) -> ShellFormatOptions {
  let indent_style = if config.use_tabs {
    IndentStyle::Tab
  } else {
    IndentStyle::Space
  };
  ShellFormatOptions::default()
    .with_dialect(resolve_dialect(config.dialect, input_text, file_path))
    .with_indent_style(indent_style)
    .with_indent_width(config.indent_width)
    .with_binary_next_line(config.binary_next_line)
    .with_switch_case_indent(config.switch_case_indent)
    .with_space_redirects(config.space_redirects)
    .with_keep_padding(config.keep_padding)
    .with_function_next_line(config.function_next_line)
    .with_never_split(config.never_split)
    .with_simplify(config.simplify)
    .with_minify(config.minify)
}

fn resolve_dialect(configured: Dialect, source: &str, file_path: &Path) -> ShellDialect {
  if configured != Dialect::Auto {
    return configured.into();
  }

  // When the first line has a shebang, let upstream determine the dialect.
  if source.starts_with("#!") {
    return ShellDialect::Auto;
  }

  // Upstream `shuck-formatter` falls back to Bash for extensionless files.
  // We help detect known zsh dotfiles when no shebang exists.
  if matches!(
    file_path.file_name().and_then(|n| n.to_str()),
    Some(".zshrc" | ".zshenv" | ".zprofile" | ".zlogin" | ".zlogout")
  ) {
    return ShellDialect::Zsh;
  }

  ShellDialect::Auto
}

/// Clamps the range to the text and widens it to the nearest char boundaries.
fn clamp_to_char_boundaries(text: &str, range: Range<usize>) -> Range<usize> {
  let mut end = range.end.min(text.len());
  let mut start = range.start.min(end);
  while !text.is_char_boundary(start) {
    start -= 1;
  }
  while !text.is_char_boundary(end) {
    end += 1;
  }
  start..end
}
