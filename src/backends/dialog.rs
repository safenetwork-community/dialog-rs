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
    title: Option<String>,
    label_helpbutton: Option<String>,
    label_extrabutton: Option<String>,
    insecure: bool,
    width: String,
    height: String,
}

impl Dialog {
    /// Creates a new `Dialog` instance without configuration.
    pub fn new() -> Dialog {
        Default::default()
    }

    /// Sets the backtitle for the dialog box.
    ///
    /// The backtitle is displayed on the backdrop, at the top of the screen.
    pub fn set_backtitle(&mut self, backtitle: impl Into<String>) {
        self.backtitle = Some(backtitle.into());
    }

    /// Sets the title for the dialog box.
    ///
    /// The title is displayed in the box, at the top.
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = Some(title.into());
    }

    /// Sets the help button for the dialog box.
    ///
    /// The help button is displayed after the OK and CANCEL button.
    pub fn set_helpbutton(&mut self, label: impl Into<String>) {
        self.label_helpbutton = Some(label.into());
    }

    /// Sets the extra button for the dialog box.
    ///
    /// The extra button is displayed between the OK and CANCEL button.
    pub fn set_extrabutton(&mut self, label: impl Into<String>) {
        self.label_extrabutton = Some(label.into());
    }

    /// Set the password input of the dialog box to insecure.
    ///
    /// At the insecure level, input will be visible in asterisks per character.
    /// At the secure level, any input will not be visible at all.
    pub fn set_insecure(&mut self, insecure: bool) {
        self.insecure = insecure;
    }

    /// Sets the height of the dialog box.
    ///
    /// The height is given in characters.  The actual height of the dialog box might be higher
    /// than the given height if the content would not fit otherwise.  The default height is zero.
    pub fn set_height(&mut self, height: u32) {
        self.height = height.to_string();
    }

    /// Sets the width of the dialog box.
    ///
    /// The width is given in characters.  The actual width of the dialog box might be higher than
    /// the given width if the content would not fit otherwise.  The default width is zero.
    pub fn set_width(&mut self, width: u32) {
        self.width = width.to_string();
    }

    fn execute(
        &self,
        boxtype: &str,
        boxtype_arg: &Option<String>,
        args: Vec<&str>,
    ) -> Result<process::Output> {
        let mut command = process::Command::new("dialog");
        command.stdin(process::Stdio::inherit());
        command.stdout(process::Stdio::inherit());

        let mut common_options: Vec<&str> = Vec::new();

        if let Some(ref backtitle) = self.backtitle {
            common_options.push("--backtitle");
            common_options.push(backtitle);
        }

        if let Some(ref title) = self.title {
            common_options.push("--title");
            common_options.push(title);
        } 

        if let Some(ref label_helpbutton) = self.label_helpbutton {
            common_options.push("--help-button");
            common_options.push("--help-label");
            common_options.push(label_helpbutton);
        }

        if let Some(ref label_extrabutton) = self.label_extrabutton {
            common_options.push("--extra-button");
            common_options.push("--extra-label");
            common_options.push(label_extrabutton);
        }

        if self.insecure {
            common_options.push("--insecure");
        } 

        command.args(common_options);
        command.arg(boxtype);
        
        if let Some(ref boxtype_arg) = boxtype_arg {
            command.arg(boxtype_arg);
        }

        command.arg(&self.height);
        command.arg(&self.width);
        command.args(args);

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
            title: None,
            label_helpbutton: None,
            label_extrabutton: None,
            insecure: false,
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
            255 => Ok(Choice::Escape),
            _ => Err(Error::from(("dialog", status))),
        }
    } else {
        Err(Error::from(("dialog", status)))
    }
}

fn get_stdchoice(output: process::Output) -> Result<(Choice, Option<String>)> {
    if let Some(code) = output.status.code() {
        let output_dialog = String::from_utf8(output.stderr).map(Some).unwrap();
        match code {
            0 => Ok((Choice::Yes, output_dialog)),
            1 => Ok((Choice::No, output_dialog)),
            2 => Ok((Choice::Help, output_dialog)),
            3 => Ok((Choice::Extra, output_dialog)),
            255 => Ok((Choice::Escape, output_dialog)),
            _ => Err(Error::from(("dialog", output.status))),
        } 
    } else { 
        Err(Error::from(("dialog", output.status)))
    }
}

impl super::Backend for Dialog {
    fn show_file_selection(&self, file_selection: &FileSelection) -> Result<(Choice, Option<String>)> {
        let dir = file_selection.path_to_string().ok_or("path not valid")?;
        self.execute("--fselect", &Some(dir), vec![])
            .and_then(get_stdchoice)
    }

    fn show_input(&self, input: &Input) -> Result<(Choice, Option<String>)> {
        let mut args: Vec<&str> = Vec::new();
        if let Some(ref default) = input.default {
            args.push(default);
        }
        self.execute("--inputbox", &Some(input.text.clone()), args)
            .and_then(get_stdchoice)
    }

    fn show_menu(&self, menu: &Menu) -> Result<(Choice, Option<String>)> {
        let mut args: Vec<&str> = Vec::new();
        let menu_height: String = menu.menu_height.to_string();
        args.push(menu_height.as_str());
        let menu_list: Vec<&str> = menu.list.iter().map(AsRef::as_ref).collect();
        args.extend(menu_list);

        self.execute("--menu", &Some(menu.text.clone()), args)
            .and_then(get_stdchoice)
    }

    fn show_message(&self, message: &Message) -> Result<()> {
        self.execute("--msgbox", &Some(message.text.clone()), vec![])
            .and_then(|output| require_success(output.status))
            .map(|_| ())
    }

    fn show_password(&self, password: &Password) -> Result<(Choice, Option<String>)> {
        self.execute("--passwordbox", &Some(password.text.clone()), vec![])
            .and_then(get_stdchoice)
    }

    fn show_question(&self, question: &Question) -> Result<Choice> {
        self.execute("--yesno", &Some(question.text.clone()), vec![])
            .and_then(|output| get_choice(output.status))
    }
}
