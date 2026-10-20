use std::fmt;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

// NOTE:
// dprint is a customizable formatter platform. We should expose options here
// when upstream `shuck-formatter` provides new formatting settings.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
  pub dialect: Dialect,
  pub indent_width: u8,
  pub use_tabs: bool,
  pub binary_next_line: bool,
  pub switch_case_indent: bool,
  pub space_redirects: bool,
  pub keep_padding: bool,
  pub function_next_line: bool,
  pub never_split: bool,
  pub simplify: bool,
  pub minify: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Dialect {
  #[default]
  Auto,
  Bash,
  Posix,
  Mksh,
  Zsh,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseDialectError(String);

impl fmt::Display for ParseDialectError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Invalid dialect: '{}'. Expected 'auto', 'bash', 'posix', 'mksh', or 'zsh'.",
      self.0
    )
  }
}

impl std::error::Error for ParseDialectError {}

impl FromStr for Dialect {
  type Err = ParseDialectError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "auto" => Ok(Dialect::Auto),
      "bash" => Ok(Dialect::Bash),
      "posix" | "sh" | "dash" | "ksh" => Ok(Dialect::Posix),
      "mksh" => Ok(Dialect::Mksh),
      "zsh" => Ok(Dialect::Zsh),
      _ => Err(ParseDialectError(s.to_string())),
    }
  }
}

impl From<Dialect> for shuck_formatter::ShellDialect {
  fn from(dialect: Dialect) -> Self {
    match dialect {
      Dialect::Auto => shuck_formatter::ShellDialect::Auto,
      Dialect::Bash => shuck_formatter::ShellDialect::Bash,
      Dialect::Posix => shuck_formatter::ShellDialect::Posix,
      Dialect::Mksh => shuck_formatter::ShellDialect::Mksh,
      Dialect::Zsh => shuck_formatter::ShellDialect::Zsh,
    }
  }
}
