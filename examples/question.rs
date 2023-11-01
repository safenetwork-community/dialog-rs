
// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

use dialog::DialogBox;

fn main() -> dialog::Result<()> {
    let choice = dialog::Question::new("Do you want to continue?").show()?;
    println!("The user chose: {:?}", choice);
    Ok(())
}
