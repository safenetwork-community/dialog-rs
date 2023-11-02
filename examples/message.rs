// Copyright (C) 2019 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: MIT

use dialog::DialogBox;

fn main() -> dialog::Result<()> {
    dialog::Message::new("This is a message.").show()?;

    dialog::Message::new("This is a message.")
        .title("And this is a title:")
        .show()
}
