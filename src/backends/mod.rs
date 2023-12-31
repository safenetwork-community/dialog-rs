// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

mod dialog;

pub use crate::backends::dialog::Dialog;

// use std::env;
// use std::path;

use crate::Result;

/// A dialog backend.
///
/// A dialog backend is a program that can be used to display dialog boxes.  Use the
/// [`default_backend`][] function to create a new instance of the default backend, or choose a
/// backend and create an instance manually.  To use a backend, pass it to the [`show_with`][]
/// method of a dialog box.
///
/// [`default_backend`]: ../fn.default_backend.html
/// [`show_with`]: ../trait.DialogBox.html#method.show_with
pub trait Backend {
    /// Shows the given file selection dialog and returns the button choice and file name selection.
    fn show_file_selection(&self, file_selection: &super::FileSelection) -> Result<(super::Choice, Option<String>)>;

    /// Shows a form of labels and text fields and returns the button choice and inputs.
    fn show_form(&self, form: &super::Form) -> Result<(super::Choice, Option<String>)>;

    /// Shows a progress bar dialog.
    fn show_gauge(&self, gauge: &super::Gauge) -> Result<()>;

    /// Shows a form of labels and text fields and returns the button choice and inputs.
    fn show_mixed_form(&self, form: &super::MixedForm) -> Result<(super::Choice, Option<String>)>;

    /// Shows a progress bar dialog with items.
    fn show_mixed_gauge(&self, guage: &super::MixedGauge) -> Result<()>;

    /// Shows the given input dialog and returns the button choice and input.
    fn show_input(&self, input: &super::Input) -> Result<(super::Choice, Option<String>)>;

    /// Shows the given menu dialog and returns the button choice and menu item selection.
    fn show_menu(&self, menu: &super::Menu) -> Result<(super::Choice, Option<String>)>;

    /// Shows the given message dialog.
    fn show_message(&self, message: &super::Message) -> Result<()>;

    /// Shows the given password dialog and returns the button choice and password.
    fn show_password(&self, password: &super::Password) -> Result<(super::Choice, Option<String>)>;

    /// Shows a form of password fields and returns the button choice and passwords.
    fn show_password_form(&self, form: &super::PasswordForm) -> Result<(super::Choice, Option<String>)>;

    /// Shows the given question dialog and returns the choice.
    fn show_question(&self, question: &super::Question) -> Result<super::Choice>;
}

/*
pub(crate) fn is_available(name: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for part in path.split(':') {
            if path::Path::new(part).join(name).exists() {
                return true;
            }
        }
    }
    false
}
*/

pub(crate) fn from_str(s: &str) -> Option<Box<dyn Backend>> {
    match s.to_lowercase().as_ref() {
        "dialog" => Some(Box::new(Dialog::new())),
        _ => None,
    }
}
