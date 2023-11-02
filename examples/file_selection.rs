// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

use dialog::DialogBox;

fn main() -> dialog::Result<()> {
    let choice = dialog::FileSelection::new("Please select a file")
        .title("File Chooser Example (Open)")
        .path("/etc")
        .show()?;
    println!("The user chose: {:?}", choice);

    let choice = dialog::FileSelection::new("Please select a file")
        .title("File Chooser Example (Save)")
        .mode(dialog::FileSelectionMode::Save)
        .path("/etc")
        .show()?;
    println!("The user chose: {:?}", choice);

    Ok(())
}
