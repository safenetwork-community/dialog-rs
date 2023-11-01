// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

use dialog::DialogBox;

fn main() -> dialog::Result<()> {
    let password = dialog::Password::new("Please enter a new password")
        .title("Password")
        .show()?;
    match password {
        Some(password) => println!("Your new password is: {}", password),
        None => println!("You do not want to have a password."),
    };
    Ok(())
}
