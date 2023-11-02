// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

use dialog::DialogBox;

fn main() -> dialog::Result<()> {
    let input1 = dialog::Input::new("Please enter something").show()?;
    let input2 = dialog::Input::new("Please enter something")
        .title("Input form")
        .show()?;
    let input3 = dialog::Input::new("Please enter something with a default")
        .title("Input form")
        .default("input")
        .show()?;

    println!("Input 1: {:?}", input1);
    println!("Input 2: {:?}", input2);
    println!("Input 3: {:?}", input3);
    Ok(())
}
