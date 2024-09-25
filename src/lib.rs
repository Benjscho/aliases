use std::{io::Write, str::FromStr};

use anyhow::{bail, Context, Result};

pub fn write_aliases<'a>(
    shell: Shell,
    aliases: impl IntoIterator<Item = (&'a str, &'a str)>,
) -> Result<()> {
    let home = dirs::home_dir().context("User does not have a home dir.")?;
    let config = match shell {
        Shell::Bash => home.join(".bashrc"),
        Shell::Zsh => home.join(".zshrc"),
        Shell::Fish => home.join(".config/fish/config.fish"),
    };

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(config)?;

    for (alias, command) in aliases {
        let alias_line = match shell {
            Shell::Bash | Shell::Zsh => format!("alias {}='{}'", alias, command),
            Shell::Fish => format!("alias {}='{}'", alias, command.replace("'", "\\''")),
        };
        writeln!(file, "{}", alias_line)?;
    }

    Ok(())
}

#[derive(Debug, PartialEq)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}

impl FromStr for Shell {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bash" => Ok(Shell::Bash),
            "zsh" => Ok(Shell::Zsh),
            "fish" => Ok(Shell::Fish),
            _ => bail!("Unrecognised shell type: {}", s),
        }
    }
}

impl Shell {
    pub fn from_str(shell: &str) -> Option<Self> {
        match shell.to_lowercase().as_str() {
            "bash" => Some(Shell::Bash),
            "zsh" => Some(Shell::Zsh),
            "fish" => Some(Shell::Fish),
            _ => None,
        }
    }
}

mod tests {
    // TODO: Add tests validating paths are written correctly
}
