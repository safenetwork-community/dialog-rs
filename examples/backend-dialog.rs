// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

use dialog::backends;
use dialog::DialogBox;

fn main() -> dialog::Result<()> {
    let mut backend = backends::Dialog::new();

    dialog::Message::new("This is a message.")
        .title("And this is a title:")
        .show_with(&backend)?;

    backend.set_backtitle("Backtitle");
    dialog::Message::new("This is a message.")
        .title("And this is a title:")
        .show_with(&backend)?;

    backend.set_width(100);
    backend.set_height(10);
    dialog::Message::new("This is a message with a fixed size.")
        .title("And this is a title:")
        .show_with(&backend)
}
