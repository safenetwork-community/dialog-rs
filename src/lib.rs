// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

#![warn(missing_docs, rust_2021_compatibility, unused)]

//! Displays dialog boxes using various backends.
//!
//! The `dialog` crate can be used to display different types of dialog boxes.  The supported types
//! are:
//! - [`FileSelection`][]: a file chooser dialog box
//! - [`Form`][]: a form
//! - [`Gauge`][]: a gauge
//! - [`Input`][]: a text input dialog
//! - [`Menu`][]: a menu selection box
//! - [`Message`][]: a simple message box
//! - [`MixedForm`][]: a mixed form
//! - [`MixedGauge`][]: a mixed gauge
//! - [`Password`][]: a password input dialog
//! - [`PasswordForm`][]: a password form
//! - [`Question`][]: a question dialog box
//!
//! These dialog boxes can be displayed using only one type of backend:
//! - [`Dialog`][]: uses `dialog` to display ncurses-based dialog boxes (requires the external
//!   `dialog` tool)
//!
//! You can let `dialog` choose the backend by calling the [`show`][] method on a dialog box.  If
//! you want to choose the backend yourself, create a backend instance and pass it to
//! [`show_with`][].  You can also use the [`default_backend`][] function to create a backend.
//!
//! # Examples
//!
//! Show a message box using the default backend:
//!
//! ```no_run
//! use dialog::DialogBox;
//!
//! dialog::Message::new("Did you know that I am using the dialog crate?")
//!     .show()
//!     .expect("Could not display dialog box");
//! ```
//!
//! Show a message box using the [`Dialog`][] backend with customized settings:
//!
//! ```no_run
//! use dialog::DialogBox;
//!
//! let mut backend = dialog::backends::Dialog::new();
//! backend.set_backtitle("dialog demo");
//! backend.set_title("box title");
//! backend.set_width(100);
//! backend.set_height(10);
//! dialog::Message::new("Did you know that I am using the dialog crate?")
//!     .show_with(&backend)
//!     .expect("Could not display dialog box");
//! ```
//!
//! Query a string from the user:
//!
//! ```no_run
//! use dialog::DialogBox;
//!
//! let name = dialog::Input::new("Please enter your name")
//!     .show()
//!     .expect("Could not display dialog box");
//! match name {
//!     Some(name) => println!("Hello {}!", name),
//!     None => println!("Hello stranger!"),
//! };
//! ```
//!
//! [`Dialog`]: backends/struct.Dialog.html
//! [`FileSelection`]: struct.FileSelection.html
//! [`Input`]: struct.Input.html
//! [`Message`]: struct.Message.html
//! [`Password`]: struct.Password.html
//! [`Question`]: struct.Question.html
//! [`default_backend`]: fn.default_backend.html
//! [`show`]: trait.DialogBox.html#method.show
//! [`show_with`]: trait.DialogBox.html#method.show_with

mod error;

/// Backends that display dialog boxes.
///
/// All backends implement the [`Backend`][] trait.  Some backends might provide additional
/// settings.  For a list of supported backends, see the [top-level crate documentation](./..) or
/// the [list of structs in this module](#structs).
///
/// [`Backend`]: trait.Backend.html
pub mod backends;

use std::{
    env,
    path::{Path, PathBuf},
};

pub use crate::error::{Error, Result};

/// A dialog box that can be shown using a backend.
///
/// Some dialog boxes might return data of the type `Output`.
pub trait DialogBox {
    /// The type of the data returned by the dialog box.
    type Output;

    /// Shows this dialog box using the default backend and returns the output.
    ///
    /// `box.show()` is a shorthand for `box.show_with(default_backend())`.
    fn show(&self) -> Result<Self::Output> {
        self.show_with(default_backend())
    }

    /// Shows this dialog box using the given backend and returns the output.
    fn show_with<B>(&self, backend: impl AsRef<B>) -> Result<Self::Output>
    where
        B: backends::Backend + ?Sized;
}

/// A menu box.
///
/// This dialog box displays a menu.
///
/// # Example
///
/// ```no_run
/// use dialog::DialogBox;
///
/// let choice = dialog::Menu::new("Please choose one of the following items..", 
/// 10, vec![["item1","value1"], ["item2","value2"]])
///     .show()
///     .expect("Could not display menu box");
/// ```
pub struct Menu {
    text: String,
    menu_height: u32,
    list: Vec<String>,
}

impl Menu {
    /// Creates a new question dialog with the given text.
    pub fn new(text: impl Into<String>, menu_height: u32, list: Vec<[String; 2]>) -> Menu {
        Menu {
            text: text.into(),
            menu_height: menu_height,
            list: list.into_iter().flatten().collect(),
        }
    }
}

impl DialogBox for Menu {
    type Output = (Choice, Option<String>);

    fn show_with<B>(&self, backend: impl AsRef<B>) -> Result<Self::Output>
    where
        B: backends::Backend + ?Sized,
    {
        backend.as_ref().show_menu(self)
    }
}



/// A message box.
///
/// This dialog box displays a text and has a single OK button.  It does not
/// produce any output.
///
/// # Example
///
/// ```no_run
/// use dialog::DialogBox;
///
/// dialog::Message::new("The operation was successful.")
///     .show()
///     .expect("Could not display dialog box");
/// ```
pub struct Message {
    text: String,
}

impl Message {
    /// Creates a new message box with the given text.
    pub fn new(text: impl Into<String>) -> Message {
        Message {
            text: text.into(),
        }
    }
}

impl DialogBox for Message {
    type Output = ();

    fn show_with<B>(&self, backend: impl AsRef<B>) -> Result<Self::Output>
    where
        B: backends::Backend + ?Sized,
    {
        backend.as_ref().show_message(self)
    }
}

/// A dialog box with a text input field.
///
/// This dialog box displays a text and an input field.  It returns the text entered by the user or
/// `None` if the user cancelled the dialog.
///
/// # Example
///
/// ```no_run
/// use dialog::DialogBox;
///
/// let name = dialog::Input::new("Please enter your name")
///     .show()
///     .expect("Could not display dialog box");
/// match name {
///     Some(name) => println!("Hello {}!", name),
///     None => println!("Hello stranger!"),
/// };
/// ```
pub struct Input {
    text: String,
    default: Option<String>,
}

impl Input {
    /// Creates a new input dialog box with the given text.
    pub fn new(text: impl Into<String>) -> Input {
        Input {
            text: text.into(),
            default: None,
        }
    }

    /// Sets the default value of this input box.
    ///
    /// This method returns a reference to `self` to enable chaining.
    pub fn default(&mut self, default: impl Into<String>) -> &mut Input {
        self.default = Some(default.into());
        self
    }
}

impl DialogBox for Input {
    type Output = (Choice, Option<String>);

    fn show_with<B>(&self, backend: impl AsRef<B>) -> Result<Self::Output>
    where
        B: backends::Backend + ?Sized,
    {
        backend.as_ref().show_input(self)
    }
}

/// A dialog box with a password input field.
///
/// This dialog box displays a text and a password input field.  It returns the password entered by
/// the user or `None` if the user cancelled the dialog.
///
/// # Example
///
/// ```no_run
/// use dialog::DialogBox;
///
/// let password = dialog::Password::new("Please enter a new password")
///     .show()
///     .expect("Could not display dialog box");
/// match password {
///     Some(password) => println!("Your new password is: {}", password),
///     None => println!("You do not want to have a password."),
/// };
/// ```
pub struct Password {
    text: String,
}

impl Password {
    /// Creates a new password dialog box with the given text.
    pub fn new(text: impl Into<String>) -> Password {
        Password {
            text: text.into(),
        }
    }
}

impl DialogBox for Password {
    type Output = (Choice, Option<String>);

    fn show_with<B>(&self, backend: impl AsRef<B>) -> Result<Self::Output>
    where
        B: backends::Backend + ?Sized,
    {
        backend.as_ref().show_password(self)
    }
}

/// A user choise in a dialog box.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Choice {
    /// The ok/yes button.
    Yes,
    /// The no button.
    No,
    /// The cancel button.
    Cancel,
    /// The extra button.
    Extra,
    /// The help button
    Help, 
    /// The escape button.
    Escape,
}

/// A question dialog box.
///
/// This dialog box displays a text and has a yes and a no button.  The
/// output is the button presed by the user, or Cancel if the dialog has been cancelled.
///
/// # Example
///
/// ```no_run
/// use dialog::DialogBox;
///
/// let choice = dialog::Question::new("Do you want to continue?")
///     .show()
///     .expect("Could not display dialog box");
/// println!("The user chose: {:?}", choice);
/// ```
pub struct Question {
    text: String,
}

impl Question {
    /// Creates a new question dialog with the given text.
    pub fn new(text: impl Into<String>) -> Question {
        Question {
            text: text.into(),
        }
    }
}

impl DialogBox for Question {
    type Output = Choice;

    fn show_with<B>(&self, backend: impl AsRef<B>) -> Result<Self::Output>
    where
        B: backends::Backend + ?Sized,
    {
        backend.as_ref().show_question(self)
    }
}

/// The type of a file selection dialog.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FileSelectionMode {
    /// An Open File dialog, meaning that the user can only select an existing file.
    Open,
    /// A Save File dialog, meaning that the user is allowed to select a non-existing file.
    Save,
}

/// A file chooser dialog box.
///
/// This dialog box opens a file choser in the specified path.  If the path
/// is not specified, it defaults to the user’s home directory.
///
/// The backends might support multiple operation modes, for example open or save dialogs.  You can
/// select a mode using the [`FileSelectionMode`][] enum, though the backend might ignore the mode
/// and just display a simple file dialog.  Per default, the mode is set to `Open`.
///
/// # Example
///
/// ```no_run
/// use dialog::DialogBox;
///
/// let choice = dialog::FileSelection::new("Please select a file")
///     .path("/home/user/Downloads")
///     .show()
///     .expect("Could not display dialog box");
/// println!("The user chose: {:?}", choice);
/// ```
///
/// [`FileSelectionMode`]: enum.FileSelectionMode.html

#[allow(dead_code)]
pub struct FileSelection {
    text: String,
    path: Option<PathBuf>,
    mode: FileSelectionMode,
}

impl FileSelection {
    /// Creates a new file chooser with the given path.
    pub fn new(text: impl Into<String>) -> FileSelection {
        FileSelection {
            text: text.into(),
            path: dirs::home_dir(),
            mode: FileSelectionMode::Open,
        }
    }

    /// Sets the path of this file chooser dialog box.
    ///
    /// This method returns a reference to `self` to enable chaining.
    pub fn path(&mut self, path: impl AsRef<Path>) -> &mut FileSelection {
        self.path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Gets the path of this file chooser dialog box.
    ///
    /// This method returns the validated directory as a `String`.
    pub fn path_to_string(&self) -> Option<String> {
        match self.path {
            Some(ref path) if path.is_dir() => {
                // The backends expect a trailing / after the directory
                path.to_str().map(|s| s.to_string() + "/")
            }
            _ => None,
        }
    }

    /// Sets the operation mode of the file chooser.
    ///
    /// This method returns a reference to `self` to enable chaining.
    pub fn mode(&mut self, mode: FileSelectionMode) -> &mut FileSelection {
        self.mode = mode;
        self
    }
}

impl DialogBox for FileSelection {
    type Output = (Choice, Option<String>);

    fn show_with<B>(&self, backend: impl AsRef<B>) -> Result<Self::Output>
    where
        B: backends::Backend + ?Sized,
    {
        backend.as_ref().show_file_selection(self)
    }
}

/// Creates a new instance of the default backend.
///
/// The following steps are performed to determine the default backend:
/// - If the `DIALOG` environment variable is set to a valid backend name, this backend is used.
///   A valid backend name is the name of a struct in the `backends` module implementing the
///   `Backend` trait in any case.
/// - If the [`Dialog`][] backend is available, it is used.
///
/// [`Dialog`]: backends/struct.Dialog.html
pub fn default_backend() -> Box<dyn backends::Backend> {
    if let Ok(backend) = env::var("DIALOG") {
        if let Some(backend) = backends::from_str(&backend) {
            return backend;
        }
    }

    Box::new(backends::Dialog::new())
}

/// A gauge box.
///
/// A guage box displays a progress bar.  
///
/// # Example
///
/// ```no_run
/// use dialog::Gauge;
///
/// dialog::Gauge::new("progress...")
///     .show()
///     .expect("Could not display dialog box");
/// ```
pub struct Gauge {
    text: String,
    percent: u8,
}

impl Gauge {
    /// Creates a new message box with the given text.
    pub fn new(text: impl Into<String>, percent: u8) -> Gauge {
        Gauge {
            text: text.into(),
            percent: percent,
        }
    }
}

impl DialogBox for Gauge {
    type Output = ();

    fn show_with<B>(&self, backend: impl AsRef<B>) -> Result<Self::Output>
    where
        B: backends::Backend + ?Sized,
    {
        backend.as_ref().show_gauge(self)
    }
}

/// A mixed gauge box.
///
/// A guage box displays a progress bar.  
///
/// # Example
///
/// ```no_run
/// use dialog::MixedGauge;
///
/// dialog::MixedGauge::new("progress...")
///     .show()
///     .expect("Could not display dialog box");
/// ```
pub struct MixedGauge {
    text: String,
    percent: u8,
}

impl MixedGauge {
    /// Creates a new message box with the given text.
    pub fn new(text: impl Into<String>, percent: u8) -> MixedGauge {
        MixedGauge {
            text: text.into(),
            percent: percent,
        }
    }
}

impl DialogBox for MixedGauge {
    type Output = ();

    fn show_with<B>(&self, backend: impl AsRef<B>) -> Result<Self::Output>
    where
        B: backends::Backend + ?Sized,
    {
        backend.as_ref().show_mixed_gauge(self)
    }
}

/// A form box.
///
/// A form box displays a progress bar.  
///
/// # Example
///
/// ```no_run
/// use dialog::Form;
///
/// dialog::Form::new("progress...")
///     .show()
///     .expect("Could not display dialog box");
/// ```
pub struct Form {
    text: String,
    form_height: u32,
    list: Vec<String>,
}

impl Form {
    /// Creates a new message box with the given text.
    pub fn new(text: impl Into<String>, form_height: u32, 
        list: Vec<(String, u8, u8, String, u8, u8, u8, u8)>) -> Form {
        Form {
            text: text.into(),
            form_height: form_height,
            list: list.iter().map(|(x1, x2, x3, x4, x5, x6, x7, x8)| 
            format!("{} {} {} {} {} {} {} {}", x1, x2, x3, x4, x5, x6, x7, x8))
            .collect()
        }
    }
}

impl DialogBox for Form { 
    type Output = (Choice, Option<String>);

    fn show_with<B>(&self, backend: impl AsRef<B>) -> Result<Self::Output>
    where
        B: backends::Backend + ?Sized,
    {
        backend.as_ref().show_form(self)
    }
}

/// A mixedform box.
///
/// A mixedform box displays a form with labels
/// and text fields of different forms to be filled out.  
///
/// # Example
///
/// ```no_run
/// use dialog::MixedForm;
///
/// dialog::MixedForm::new("Please enter the information")
///     .show()
///     .expect("Could not display dialog box");
/// ```
pub struct MixedForm {
    text: String,
    form_height: u32,
    list: Vec<String>,
}

impl MixedForm {
    /// Creates a new message box with the given text.
    pub fn new(text: impl Into<String>, form_height: u32, 
        list: Vec<(String, u8, u8, String, u8, u8, u8, u8)>) -> MixedForm {
        MixedForm {
            text: text.into(),
            form_height: form_height,
            list: list.iter().map(|(x1, x2, x3, x4, x5, x6, x7, x8)| 
            format!("{} {} {} {} {} {} {} {}", x1, x2, x3, x4, x5, x6, x7, x8))
            .collect()
        }
    }
}

impl DialogBox for MixedForm {
    type Output = (Choice, Option<String>);

    fn show_with<B>(&self, backend: impl AsRef<B>) -> Result<Self::Output>
    where
        B: backends::Backend + ?Sized,
    {
        backend.as_ref().show_mixed_form(self)
    }
}

/// A password form box.
///
/// A form with password input fields.  
///
/// # Example
///
/// ```no_run
/// use dialog::PasswordForm;
///
/// dialog::PasswordForm::new("Please enter the information")
///     .show()
///     .expect("Could not display dialog box");
/// ```
pub struct PasswordForm {
    text: String,
    form_height: u32,
    list: Vec<String>,
}

impl PasswordForm {
    /// Creates a new message box with the given text.
    pub fn new(text: impl Into<String>, form_height: u32, 
        list: Vec<(String, u8, u8, String, u8, u8, u8, u8)>) -> PasswordForm {
        PasswordForm {
            text: text.into(),
            form_height: form_height,
            list: list.iter().map(|(x1, x2, x3, x4, x5, x6, x7, x8)| 
            format!("{} {} {} {} {} {} {} {}", x1, x2, x3, x4, x5, x6, x7, x8))
            .collect()
        }
    }
}

impl DialogBox for PasswordForm {
    type Output = (Choice, Option<String>);

    fn show_with<B>(&self, backend: impl AsRef<B>) -> Result<Self::Output>
    where
        B: backends::Backend + ?Sized,
    {
        backend.as_ref().show_password_form(self)
    }
}


