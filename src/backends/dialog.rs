// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

use std::process;

use crate::{Choice, Error, FileSelection, Menu, Input, Message, Password, Question, Result};

/// The `dialog` backend.
///
/// This backend uses the external `dialog` program (not to be confused with this crate also called
/// `dialog`) to display text-based dialog boxes in the terminal.
#[derive(Debug)]
pub struct Dialog {
    backtitle: Option<String>,
    width: String,
    height: String,
}

impl Dialog {
    /// Creates a new `Dialog` instance without configuration.
    pub fn new() -> Dialog {
        Default::default()
    }

    /// Sets the backtitle for the dialog boxes.
    ///
    /// The backtitle is displayed on the backdrop, at the top of the screen.
    pub fn set_backtitle(&mut self, backtitle: impl Into<String>) {
        self.backtitle = Some(backtitle.into());
    }

    /// Sets the height of the dialog boxes.
    ///
    /// The height is given in characters.  The actual height of the dialog box might be higher
    /// than the given height if the content would not fit otherwise.  The default height is zero.
    pub fn set_height(&mut self, height: u32) {
        self.height = height.to_string();
    }

    /// Sets the width of the dialog boxes.
    ///
    /// The width is given in characters.  The actual width of the dialog box might be higher than
    /// the given width if the content would not fit otherwise.  The default width is zero.
    pub fn set_width(&mut self, width: u32) {
        self.width = width.to_string();
    }

    /*
    pub(crate) fn is_available() -> bool {
        super::is_available("dialog")
    }
    */

    fn execute(
        &self,
        args: Vec<&str>,
        post_args: Vec<&str>,
        title: &Option<String>,
    ) -> Result<process::Output> {
        let mut command = process::Command::new("dialog");
        command.stdin(process::Stdio::inherit());
        command.stdout(process::Stdio::inherit());

        if let Some(ref backtitle) = self.backtitle {
            command.arg("--backtitle");
            command.arg(backtitle);
        }
        if let Some(ref title) = title {
            command.arg("--title");
            command.arg(title);
        }

        command.args(args);
        command.arg(&self.height);
        command.arg(&self.width);
        command.args(post_args);

        command.output().map_err(Error::IoError)
    }
}

impl AsRef<Dialog> for Dialog {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Default for Dialog {
    fn default() -> Self {
        Dialog {
            backtitle: None,
            height: "0".to_string(),
            width: "0".to_string(),
        }
    }
}

fn require_success(status: process::ExitStatus) -> Result<()> {
    if status.success() {
        Ok(())
    } else {
        Err(Error::from(("dialog", status)))
    }
}

fn get_choice(status: process::ExitStatus) -> Result<Choice> {
    if let Some(code) = status.code() {
        match code {
            0 => Ok(Choice::Yes),
            1 => Ok(Choice::No),
            255 => Ok(Choice::Cancel),
            _ => Err(Error::from(("dialog", status))),
        }
    } else {
        Err(Error::from(("dialog", status)))
    }
}

fn get_stderr(output: process::Output) -> Result<Option<String>> {
    if output.status.success() {
        String::from_utf8(output.stderr)
            .map(Some)
            .map_err(Error::from)
    } else if let Some(code) = output.status.code() {
        match code {
            0 => Ok(None),
            1 => Ok(None),
            255 => Ok(None),
            _ => Err(Error::from(("dialog", output.status))),
        }
    } else {
        Err(Error::from(("dialog", output.status)))
    }
}

impl super::Backend for Dialog {
    fn show_file_selection(&self, file_selection: &FileSelection) -> Result<Option<String>> {
        let dir = file_selection.path_to_string().ok_or("path not valid")?;
        let args = vec!["--fselect", &dir];
        self.execute(args, vec![], &file_selection.title)
            .and_then(get_stderr)
    }

    fn show_input(&self, input: &Input) -> Result<Option<String>> {
        let args = vec!["--inputbox", &input.text];
        let mut post_args: Vec<&str> = Vec::new();
        if let Some(ref default) = input.default {
            post_args.push(default);
        }
        self.execute(args, post_args, &input.title)
            .and_then(get_stderr)
    }

    fn show_menu(&self, menu: &Menu) -> Result<Choice> {
        let args = vec!["--menu", &menu.text];
        self.execute(args, vec![], &menu.title)
            .and_then(|output| get_choice(output.status))
    }

    fn show_message(&self, message: &Message) -> Result<()> {
        let args = vec!["--msgbox", &message.text];
        self.execute(args, vec![], &message.title)
            .and_then(|output| require_success(output.status))
            .map(|_| ())
    }

    fn show_password(&self, password: &Password) -> Result<Option<String>> {
        let args = vec!["--passwordbox", &password.text];
        let mut post_args: Vec<&str> = Vec::new();
        if password.insecure {
            post_args.push("--insecure");
        }
        self.execute(args, vec![], &password.title)
            .and_then(get_stderr)
    }

    fn show_question(&self, question: &Question) -> Result<Choice> {
        let args = vec!["--yesno", &question.text];
        self.execute(args, vec![], &question.title)
            .and_then(|output| get_choice(output.status))
    }
}
