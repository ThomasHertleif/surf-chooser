# Surf Chooser
## Setup

1. `cargo build --release`
1. cp & edit `config.toml` to `~/.config/surf-chooser/`
1. cp `target/release/surf-chooser` to `/opt/surf-chooser`
1. `sudo desktop-file-install surf-chooser.desktop --rebuild-mime-info-cache`
1. `gio mime x-scheme-handler/https surf-chooser.desktop`
