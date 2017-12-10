# advent of code 2017

<https://adventofcode.com/>

Each directory is a rust project.

You can `cd` into the directory and then run `cargo run --release` to get the solution.

You can also run `cargo test` -- most of these should have some tests.

I'm also generally recording my screen as I work on these.
If curious, you can [check out the videos on YouTube](https://www.youtube.com/playlist?list=PLtKaBKjqBNUf1KmmCUyuVITQ-YoyLZpef).

## How to record videos (on macOS)

1. Scale down resolution so the video won't be enormous
    1. Open "Displays" section of "System Preferences"
    1. Click "Scaled"
    1. Select 720p
1. Turn off notifications
    1. Option-click on the little notifications button in the menu bar to disable notifications
1. Make sure the input is loud enough
    1. Visit "Sound" section of "System Preferences"
    1. Drag Input volume to like 75%. This is a one time thing that I just didn't know about, and my first few videos were quiet because of it
1. Hide the menu bar
    1. Visit "General" section of "System Preferences"
    1. Check "Automatically hide and show the menu bar"
1. Prevent Firefox from leaking personal information
    1. Visit <about:config>
    1. Filter configs by `browser.urlbar.suggest`
    1. toggle `browser.urlbar.suggest.bookmark` to false
    1. toggle `browser.urlbar.suggest.history` to false
    1. toggle `browser.urlbar.suggest.searches` to false
1. Record video
    1. Open Quicktime
    1. Select File > New Screen Recording
    1. Click the little dropdown to make sure the right microphone is selected
    1. Click record
    1. Click anywhere to record the whole screen
    1. When done, click on the stop button in the menu bar
