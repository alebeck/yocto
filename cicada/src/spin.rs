//
// (c) 2019 Alexander Becker
// Released under the MIT license.
//

use std::time::Duration;

pub fn spin(message: String) -> spinner::SpinnerHandle {

    let frames = vec![
        "·  ",
        "·· ",
        "···",
        " ··",
        "  ·",
        "   "
    ];

    let handle = spinner::SpinnerBuilder::new(message)
        .spinner(frames)
        .step(Duration::from_millis(200))
        .start();

    handle
}