# snake-rs

A simple snake game written in [Rust](https://www.rust-lang.org/) using the [ggez](https://ggez.rs/) crate.

![gameplay gif](gameplay.gif)

## Usage

Use the arrow or hjkl keys to move the snake around.

Press Q to quit the game.

## Command Line Options

```
USAGE:
    snake-rs [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --background-color <COLOR>       Sets the window background color [default: #000000]
    -f, --food-color <COLOR>             Sets the color of the food [default: #AADDFF]
        --grid-cell-height <NUMBER>      Sets the height of a grid cell [default: 20]
        --grid-cell-width <NUMBER>       Sets the width of a grid cell [default: 20]
    -h, --grid-height <NUMBER>           Sets the height of the grid [default: 20]
    -w, --grid-width <NUMBER>            Sets the width of the grid [default: 32]
    -s, --snake-body-color <COLOR>       Sets the color of the snake body [default: #CCCCCC]
    -n, --snake-head-color <COLOR>       Sets the color of the snake head [default: #FFFFFF]
    -u, --updates-per-second <NUMBER>    Sets how often the game should update per second [default: 8]
```
