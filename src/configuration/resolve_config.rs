use super::Configuration;
use super::Dialect;
use dprint_core::configuration::*;

/// Resolves configuration from a collection of key value strings.
///
/// # Example
///
/// ```
/// use dprint_core::configuration::ConfigKeyMap;
/// use dprint_core::configuration::resolve_global_config;
/// use dprint_plugin_shuck::configuration::resolve_config;
///
/// let mut config_map = ConfigKeyMap::new(); // get a collection of key value pairs from somewhere
/// let global_config_result = resolve_global_config(&mut config_map);
///
/// // check global_config_result.diagnostics here...
///
/// let config_map = ConfigKeyMap::new(); // get a collection of k/v pairs from somewhere
/// let config_result = resolve_config(
///     config_map,
///     &global_config_result.config
/// );
///
/// // check config_result.diagnostics here and use config_result.config
/// ```
pub fn resolve_config(
  config: ConfigKeyMap,
  global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<Configuration> {
  let mut diagnostics = Vec::new();
  let mut config = config;
  // NOTE:
  // Always use defaults from the `shuck-formatter` crate to keep this plugin simple,
  // even if defaults between shfmt and shuck-formatter differ in the future.
  let defaults = shuck_formatter::ShellFormatOptions::default();

  let resolved_config = Configuration {
    dialect: get_value(&mut config, "dialect", Dialect::default(), &mut diagnostics),
    indent_width: get_value(
      &mut config,
      "indentWidth",
      global_config.indent_width.unwrap_or(defaults.indent_width()),
      &mut diagnostics,
    ),
    use_tabs: get_value(
      &mut config,
      "useTabs",
      global_config
        .use_tabs
        .unwrap_or(defaults.indent_style() == shuck_formatter::IndentStyle::Tab),
      &mut diagnostics,
    ),
    binary_next_line: get_value(
      &mut config,
      "binaryNextLine",
      defaults.binary_next_line(),
      &mut diagnostics,
    ),
    switch_case_indent: get_value(
      &mut config,
      "switchCaseIndent",
      defaults.switch_case_indent(),
      &mut diagnostics,
    ),
    space_redirects: get_value(
      &mut config,
      "spaceRedirects",
      defaults.space_redirects(),
      &mut diagnostics,
    ),
    keep_padding: get_value(&mut config, "keepPadding", defaults.keep_padding(), &mut diagnostics),
    function_next_line: get_value(
      &mut config,
      "functionNextLine",
      defaults.function_next_line(),
      &mut diagnostics,
    ),
    never_split: get_value(&mut config, "neverSplit", defaults.never_split(), &mut diagnostics),
    simplify: get_value(&mut config, "simplify", defaults.simplify(), &mut diagnostics),
    minify: get_value(&mut config, "minify", defaults.minify(), &mut diagnostics),
  };

  diagnostics.extend(get_unknown_property_diagnostics(config));

  ResolveConfigurationResult {
    config: resolved_config,
    diagnostics,
  }
}
