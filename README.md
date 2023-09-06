# Meteocli
Show meteorology forecast on terminal in the format of a table

## Instalation
To install run the following commands:
```sh
git clone https://github.com/drcor/meteocli.git
cargo build --release
cargo install --path .
```
Then create a file at `~/.config/` named `meteocli.toml`, and paste the text
bellow, changing to the `latitude`, `longitude` and `elevation` you want.

```toml
# My location
latitude =  0.00000
longitude = 0.00000
elevation = 90  # is used for statistical downscalling
```
