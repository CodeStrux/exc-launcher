use clap::CommandFactory;

use crate::cli::{Cli, CompleteKind};
use crate::config::Config;

const ZSH_TEMPLATE: &str = include_str!("exc.zsh");

/// The zsh completion script printed by `exc completions zsh`. The static
/// parts (flags, value completers) live in `exc.zsh`; the subcommand list is
/// filled in from the clap definition so it can't drift out of sync with
/// `--help`. Config-defined command/profile names are looked up at Tab time
/// through `exc __complete`, see [`candidates`].
pub fn zsh_script() -> String {
    let mut cmd = Cli::command();
    cmd.build();
    let subcommands: Vec<String> = cmd
        .get_subcommands()
        .filter(|s| !s.is_hide_set())
        .map(|s| {
            let about = s.get_about().map(|a| a.to_string()).unwrap_or_default();
            format!("    {}", zsh_single_quote(&describe_entry(s.get_name(), &about)))
        })
        .collect();
    ZSH_TEMPLATE.replace("@SUBCOMMANDS@", &subcommands.join("\n"))
}

/// One `name:description` line per candidate, in the format zsh's
/// `_describe` reads, for the hidden `exc __complete <kind>` subcommand.
pub fn candidates(cfg: &Config, kind: CompleteKind) -> Vec<String> {
    match kind {
        CompleteKind::Commands => cfg
            .all_commands()
            .into_iter()
            .map(|(profile, cmd)| {
                let desc = if cmd.description.is_empty() {
                    format!("[{}]", profile.name)
                } else {
                    format!("[{}] {}", profile.name, cmd.description)
                };
                describe_entry(&cmd.name, &desc)
            })
            .collect(),
        CompleteKind::Profiles => cfg
            .profiles
            .iter()
            .map(|p| describe_entry(&p.name, p.description.as_deref().unwrap_or(p.display_label())))
            .collect(),
    }
}

/// `_describe` splits on the first unescaped `:`, so colons in the name are
/// escaped; the description is flattened to a single line since candidates
/// are newline-separated.
fn describe_entry(name: &str, desc: &str) -> String {
    let name = name.replace('\\', "\\\\").replace(':', "\\:");
    let desc = desc.split_whitespace().collect::<Vec<_>>().join(" ");
    if desc.is_empty() { name } else { format!("{name}:{desc}") }
}

fn zsh_single_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', r"'\''"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Config {
        let raw = std::fs::read_to_string("tests/fixtures/valid.toml").unwrap();
        toml::from_str(&raw).unwrap()
    }

    #[test]
    fn script_lists_every_visible_subcommand() {
        let script = zsh_script();
        assert!(!script.contains("@SUBCOMMANDS@"));
        for name in ["run", "list", "validate", "sysinfo", "init", "man", "completions", "help"] {
            assert!(script.contains(&format!("    '{name}:")), "missing subcommand {name}");
        }
        assert!(!script.contains("'__complete"), "hidden subcommand must not be offered");
    }

    #[test]
    fn script_covers_every_global_flag() {
        let script = zsh_script();
        for arg in Cli::command().get_arguments() {
            if let Some(long) = arg.get_long() {
                assert!(script.contains(&format!("'--{long}")), "missing global flag --{long}");
            }
        }
    }

    #[test]
    fn command_candidates_come_from_config() {
        let lines = candidates(&fixture(), CompleteKind::Commands);
        assert!(lines.iter().any(|l| l.starts_with("cert-check-online:")), "{lines:?}");
        assert_eq!(lines.len(), fixture().all_commands().len());
    }

    #[test]
    fn profile_candidates_come_from_config() {
        let cfg = fixture();
        let lines = candidates(&cfg, CompleteKind::Profiles);
        assert_eq!(lines.len(), cfg.profiles.len());
        assert!(lines[0].starts_with(&cfg.profiles[0].name));
    }

    #[test]
    fn describe_entry_escapes_colons_and_flattens_newlines() {
        assert_eq!(describe_entry("a:b", "line one\n  line two"), r"a\:b:line one line two");
        assert_eq!(describe_entry("plain", ""), "plain");
    }
}
