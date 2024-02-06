// Copyright (c) soynexz. All rights reserved.
// Check the README file in the project root for more information.

use chrono::Utc;
use env_logger::Env;
use owo_colors::{OwoColorize, Style};
use supports_color::Stream;

pub struct Logger {}

impl Logger {
  pub fn init() {
    let env: Env<'_> = Env::default().filter_or("LOG_LEVEL", "info");

    env_logger::Builder::from_env(env)
      .format(|_, record| {
        Ok({
          let timestamp: chrono::format::DelayedFormat<chrono::format::StrftimeItems<'_>> =
            Utc::now().format("%Y-%m-%d %H:%M:%S");
          let level: log::Level = record.level();
          let level_text: &str = match level {
            log::Level::Error => "ERROR",
            log::Level::Warn => "WARN ",
            log::Level::Info => "INFO ",
            log::Level::Debug => "DEBUG",
            log::Level::Trace => "TRACE",
          };

          if let Some(support) = supports_color::on(Stream::Stdout) {
            if support.has_16m || support.has_256 {
              let background_color: Style = match level {
                log::Level::Error => Style::new().on_red(),
                log::Level::Warn => Style::new().on_yellow(),
                log::Level::Info => Style::new().on_blue(),
                log::Level::Debug => Style::new().on_magenta(),
                log::Level::Trace => Style::new().on_white(),
              };
              let foreground_color: Style = match level {
                log::Level::Error => Style::new().red(),
                log::Level::Warn => Style::new().yellow(),
                log::Level::Info => Style::new().blue(),
                log::Level::Debug => Style::new().magenta(),
                log::Level::Trace => Style::new().white(),
              };

              let log_text_1: String = format!(" | {} ", level_text);
              let log_text_2: String = format!(" {} ", timestamp);

              let formatted_log: String = format!(
                "{}{}{} {}",
                log_text_1.style(background_color),
                log_text_2.on_bright_black(),
                " ".style(background_color),
                record.args().style(foreground_color)
              );

              match level {
                log::Level::Error => {
                  eprintln!("{}", formatted_log)
                }
                _ => {
                  println!("{}", formatted_log)
                }
              }
            } else {
              let formatted_log: String =
                format!("{} · {} · {}", level_text, timestamp, record.args());

              match level {
                log::Level::Error => {
                  eprintln!("{}", formatted_log)
                }
                _ => {
                  println!("{}", formatted_log)
                }
              }
            }
          }
        })
      })
      .init();
  }
}
