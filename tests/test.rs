extern crate dprint_development;
extern crate dprint_plugin_sh;

use std::path::PathBuf;
use std::sync::Arc;

use dprint_core::configuration::*;
use dprint_development::*;
use dprint_plugin_sh::configuration::Configuration;
use dprint_plugin_sh::configuration::Dialect;
use dprint_plugin_sh::configuration::resolve_config;
use dprint_plugin_sh::*;

#[test]
fn test_specs() {
  let global_config = GlobalConfiguration::default();

  run_specs(
    &PathBuf::from("./tests/specs"),
    &ParseSpecOptions {
      default_file_name: "file.sh",
    },
    &RunSpecsOptions {
      fix_failures: false,
      format_twice: true,
    },
    {
      let global_config = global_config.clone();
      Arc::new(move |file_path, file_text, range, spec_config| {
        let spec_config: ConfigKeyMap = serde_json::from_value(spec_config.clone().into()).unwrap();
        let config_result = resolve_config(spec_config, &global_config);
        ensure_no_diagnostics(&config_result.diagnostics);

        match range {
          Some(range) => format_text_range(file_path, file_text, range, &config_result.config),
          None => format_text(file_path, file_text, &config_result.config),
        }
        .map_err(|err| err.into())
      })
    },
    Arc::new(move |_file_path, _file_text, _spec_config| panic!("Plugin does not support dprint-core tracing.")),
  )
}

#[test]
fn should_have_schema_defaults_matching_resolved_config() {
  let schema: serde_json::Value = serde_json::from_str(include_str!("../deployment/schema.json")).unwrap();
  let properties = schema["properties"].as_object().unwrap();
  let config = serde_json::to_value(default_config()).unwrap();
  let config = config.as_object().unwrap();
  assert_eq!(properties.len(), config.len());
  for (key, value) in config {
    assert_eq!(&properties[key]["default"], value, "default of {key}");
  }
}

#[test]
fn should_use_global_config() {
  let global_config = GlobalConfiguration {
    indent_width: Some(4),
    use_tabs: Some(false),
    ..Default::default()
  };
  let result = resolve_config(Default::default(), &global_config);
  assert!(result.diagnostics.is_empty());
  assert_eq!(result.config.indent_width, 4);
  assert!(!result.config.use_tabs);
}

#[test]
fn should_parse_dialect() {
  assert_eq!("auto".parse::<Dialect>(), Ok(Dialect::Auto));
  assert_eq!("bash".parse::<Dialect>(), Ok(Dialect::Bash));
  assert_eq!("posix".parse::<Dialect>(), Ok(Dialect::Posix));
  assert_eq!("sh".parse::<Dialect>(), Ok(Dialect::Posix));
  assert_eq!("dash".parse::<Dialect>(), Ok(Dialect::Posix));
  assert_eq!("ksh".parse::<Dialect>(), Ok(Dialect::Posix));
  assert_eq!("mksh".parse::<Dialect>(), Ok(Dialect::Mksh));
  assert_eq!("zsh".parse::<Dialect>(), Ok(Dialect::Zsh));
  assert!("unknown".parse::<Dialect>().is_err());
}

#[test]
fn should_fail_on_bash_syntax_in_posix_sh_file() {
  // in the default auto mode, a .sh file without a shebang is parsed as posix, which doesn't support `[[ ]]`
  let result = format_text(
    &PathBuf::from("./file.sh"),
    "if [[ 1 -eq 1 ]]; then\necho foo\nfi\n",
    &default_config(),
  );
  assert!(result.unwrap_err().to_string().starts_with("Formatting failed: "));
}

#[test]
fn should_format_range() {
  let result = format_text_range(
    &PathBuf::from("./file.sh"),
    "echo   one\necho   two\n",
    11..22,
    &default_config(),
  )
  .unwrap();
  assert_eq!(result.unwrap(), "echo   one\necho two\n");
}

#[test]
fn should_format_range_with_unrelated_syntax_error() {
  let result = format_text_range(
    &PathBuf::from("./file.sh"),
    "echo   one\nif true; then\n",
    0..9,
    &default_config(),
  )
  .unwrap();
  assert_eq!(result.unwrap(), "echo one\nif true; then\n");
}

#[test]
fn should_not_change_formatted_range() {
  let result = format_text_range(
    &PathBuf::from("./file.sh"),
    "echo one\necho two\n",
    0..8,
    &default_config(),
  )
  .unwrap();
  assert_eq!(result, None);
}

#[test]
fn should_not_change_range_out_of_bounds() {
  let result = format_text_range(&PathBuf::from("./file.sh"), "echo hello\n", 100..200, &default_config()).unwrap();
  assert_eq!(result, None);
}

#[test]
fn should_not_change_range_in_incomplete_statement() {
  let result = format_text_range(&PathBuf::from("./file.sh"), "if then\n", 0..5, &default_config()).unwrap();
  assert_eq!(result, None);
}

fn default_config() -> Configuration {
  resolve_config(Default::default(), &GlobalConfiguration::default()).config
}
