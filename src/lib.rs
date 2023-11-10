// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

#![warn(missing_docs, rust_2021_compatibility, unused)]

//! Displays dialog boxes using various backends.
//!
//! The `dialog` crate can be used to display different types of dialog boxes.  The supported types
//! are:
//! - [`FileSelection`][]: a file chooser dialog box
//! - [`Input`][]: a text input dialog
//! - [`Menu`][]: a menu selection box
//! - [`Message`][]: a simple message box
//! - [`Password`][]: a password input dialog
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
//!     .title("Public Service Announcement")
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
//! backend.set_width(100);
//! backend.set_height(10);
//! dialog::Message::new("Did you know that I am using the dialog crate?")
//!     .title("Public Service Announcement")
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
//!     .title("Name")
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
/// This dialog box displays a text and an optional title and has a yes and a no button.  The
/// output is the button presed by the user, or Cancel if the dialog has been cancelled.
///
/// # Example
///
/// ```no_run
/// use dialog::DialogBox;
///
/// let choice = dialog::Menu::new("Do you want to continue?")
///     .title("Question")
///     .show()
///     .expect("Could not display dialog box");
/// println!("The user chose: {:?}", choice);
/// ```
pub struct Menu {
    text: String,
    title: Option<String>,
    menu_height: u32,
    list: Vec<String>,
}

impl Menu {
    /// Creates a new question dialog with the given text.
    pub fn new(text: impl Into<String>, menu_height: u32, list: Vec<[&str; 2]>) -> Menu {
        Menu {
            title: None,
            text: text.into(),
            menu_height: menu_height,
            list: list.into_iter().flat_map(|x| x.into_iter().map(String::from)).collect(),
        }
    }

    /// Sets the title of this question dialog box.
    ///
    /// This method returns a reference to `self` to enable chaining.
    pub fn title(&mut self, title: impl Into<String>) -> &mut Menu {
        self.title = Some(title.into());
        self
    }
}

impl DialogBox for Menu {
    type Output = Option<String>;

    fn show_with<B>(&self, backend: impl AsRef<B>) -> Result<Self::Output>
    where
        B: backends::Backend + ?Sized,
    {
        backend.as_ref().show_menu(self)
    }
}



/// A message box.
///
/// This dialog box displays a text and an optional title and has a single OK button.  It does not
/// produce any output.
///
/// # Example
///
/// ```no_run
/// use dialog::DialogBox;
///
/// dialog::Message::new("The operation was successful.")
///     .title("Success")
///     .show()
///     .expect("Could not display dialog box");
/// ```
pub struct Message {
    text: String,
    title: Option<String>,
}

impl Message {
    /// Creates a new message box with the given text.
    pub fn new(text: impl Into<String>) -> Message {
        Message {
            text: text.into(),
            title: None,
        }
    }

    /// Sets the title of this message box.
    ///
    /// This method returns a reference to `self` to enable chaining.
    pub fn title(&mut self, title: impl Into<String>) -> &mut Message {
        self.title = Some(title.into());
        self
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
///     .title("Name")
///     .show()
///     .expect("Could not display dialog box");
/// match name {
///     Some(name) => println!("Hello {}!", name),
///     None => println!("Hello stranger!"),
/// };
/// ```
pub struct Input {
    text: String,
    title: Option<String>,
    default: Option<String>,
}

impl Input {
    /// Creates a new input dialog box with the given text.
    pub fn new(text: impl Into<String>) -> Input {
        Input {
            text: text.into(),
            title: None,
            default: None,
        }
    }

    /// Sets the title of this input box.
    ///
    /// This method returns a reference to `self` to enable chaining.
    pub fn title(&mut self, title: impl Into<String>) -> &mut Input {
        self.title = Some(title.into());
        self
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
    type Output = Option<String>;

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
///     .title("Password")
///     .insecure(true)
///     .show()
///     .expect("Could not display dialog box");
/// match password {
///     Some(password) => println!("Your new password is: {}", password),
///     None => println!("You do not want to have a password."),
/// };
/// ```
pub struct Password {
    text: String,
    title: Option<String>,
    insecure: bool,
}

impl Password {
    /// Creates a new password dialog box with the given text.
    pub fn new(text: impl Into<String>) -> Password {
        Password {
            text: text.into(),
            title: None,
            insecure: false,
        }
    }

    /// Sets the title of this password dialog box.
    ///
    /// This method returns a reference to `self` to enable chaining.
    pub fn title(&mut self, title: impl Into<String>) -> &mut Password {
        self.title = Some(title.into());
        self
    }

    /// Toggles password dialog box to show the length of the password in asterisks.
    ///
    /// This method returns a reference to `self` to enable chaining.
    pub fn insecure(&mut self, insecure: bool) -> &mut Password {
        self.insecure = insecure;
        self
    }
}

impl DialogBox for Password {
    type Output = Option<String>;

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
    /// The yes button.
    Yes,
    /// The no button.
    No,
    /// The cancel button or a cancelled dialog.
    Cancel,
}

/// A question dialog box.
///
/// This dialog box displays a text and an optional title and has a yes and a no button.  The
/// output is the button presed by the user, or Cancel if the dialog has been cancelled.
///
/// # Example
///
/// ```no_run
/// use dialog::DialogBox;
///
/// let choice = dialog::Question::new("Do you want to continue?")
///     .title("Question")
///     .show()
///     .expect("Could not display dialog box");
/// println!("The user chose: {:?}", choice);
/// ```
pub struct Question {
    text: String,
    title: Option<String>,
}

impl Question {
    /// Creates a new question dialog with the given text.
    pub fn new(text: impl Into<String>) -> Question {
        Question {
            text: text.into(),
            title: None,
        }
    }

    /// Sets the title of this question dialog box.
    ///
    /// This method returns a reference to `self` to enable chaining.
    pub fn title(&mut self, title: impl Into<String>) -> &mut Question {
        self.title = Some(title.into());
        self
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
/// This dialog box opens a file choser with an optional title in the specified path.  If the path
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
///     .title("File Selection")
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
    title: Option<String>,
    path: Option<PathBuf>,
    mode: FileSelectionMode,
}

impl FileSelection {
    /// Creates a new file chooser with the given path.
    pub fn new(text: impl Into<String>) -> FileSelection {
        FileSelection {
            text: text.into(),
            title: None,
            path: dirs::home_dir(),
            mode: FileSelectionMode::Open,
        }
    }

    /// Sets the title of this file chooser dialog box.
    ///
    /// This method returns a reference to `self` to enable chaining.
    pub fn title(&mut self, title: impl Into<String>) -> &mut FileSelection {
        self.title = Some(title.into());
        self
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
    type Output = Option<String>;

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
