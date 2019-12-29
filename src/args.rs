extern crate clap;
extern crate ggez;

use crate::options::Options;
use clap::{App, Arg, ArgMatches};
use ggez::graphics;

pub fn get_options() -> Result<Options, clap::Error> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("grid-width")
                .short("w")
                .long("grid-width")
                .takes_value(true)
                .value_name("NUMBER")
                .default_value("32")
                .help("Sets the width of the grid"),
        )
        .arg(
            Arg::with_name("grid-height")
                .short("h")
                .long("grid-height")
                .takes_value(true)
                .value_name("NUMBER")
                .default_value("20")
                .help("Sets the height of the grid"),
        )
        .arg(
            Arg::with_name("grid-cell-width")
                .long("grid-cell-width")
                .takes_value(true)
                .value_name("NUMBER")
                .default_value("20")
                .help("Sets the width of a grid cell"),
        )
        .arg(
            Arg::with_name("grid-cell-height")
                .long("grid-cell-height")
                .takes_value(true)
                .value_name("NUMBER")
                .default_value("20")
                .help("Sets the height of a grid cell"),
        )
        .arg(
            Arg::with_name("background-color")
                .short("b")
                .long("background-color")
                .takes_value(true)
                .value_name("COLOR")
                .default_value("#000000")
                .help("Sets the window background color"),
        )
        .arg(
            Arg::with_name("snake-head-color")
                .short("n")
                .long("snake-head-color")
                .takes_value(true)
                .value_name("COLOR")
                .default_value("#FFFFFF")
                .help("Sets the color of the snake head"),
        )
        .arg(
            Arg::with_name("snake-body-color")
                .short("s")
                .long("snake-body-color")
                .takes_value(true)
                .value_name("COLOR")
                .default_value("#CCCCCC")
                .help("Sets the color of the snake body"),
        )
        .arg(
            Arg::with_name("food-color")
                .short("f")
                .long("food-color")
                .takes_value(true)
                .value_name("COLOR")
                .default_value("#AADDFF")
                .help("Sets the color of the food"),
        )
        .arg(
            Arg::with_name("updates-per-second")
                .short("u")
                .long("updates-per-second")
                .takes_value(true)
                .value_name("NUMBER")
                .default_value("8")
                .help("Sets how often the game should update per second"),
        )
        .get_matches();

    let grid_width = arg_to_i16(&matches, "grid-width")?;
    let grid_height = arg_to_i16(&matches, "grid-height")?;
    let grid_cell_width = arg_to_i16(&matches, "grid-cell-width")?;
    let grid_cell_height = arg_to_i16(&matches, "grid-cell-height")?;

    let background_color = arg_to_color(&matches, "background-color")?;
    let snake_head_color = arg_to_color(&matches, "snake-head-color")?;
    let snake_body_color = arg_to_color(&matches, "snake-body-color")?;
    let food_color = arg_to_color(&matches, "food-color")?;

    let millis_per_update = match arg_to_i16(&matches, "updates-per-second") {
        Ok(v) => (1.0 / v as f32 * 1000.0) as u64,
        Err(e) => return Err(e),
    };

    Ok(Options {
        grid_size: (grid_width, grid_height),
        grid_cell_size: (grid_cell_width, grid_cell_height),
        background_color,
        snake_head_color,
        snake_body_color,
        food_color,
        millis_per_update,
    })
}

fn get_invalid_number_error() -> clap::Error {
    clap::Error::with_description(
        "Unable to parse value. NUMBER must be an integer with 16 bits.",
        clap::ErrorKind::InvalidValue,
    )
}

fn arg_to_i16(matches: &ArgMatches, arg_name: &str) -> Result<i16, clap::Error> {
    let value = match matches.value_of(arg_name) {
        Some(v) => v,
        None => return Err(get_invalid_number_error()),
    };
    to_i16(value)
}

fn to_i16(value: &str) -> Result<i16, clap::Error> {
    match value.parse::<i16>() {
        Ok(v) => return Ok(v),
        Err(_) => return Err(get_invalid_number_error()),
    }
}

fn get_invalid_color_error() -> clap::Error {
    clap::Error::with_description(
        "Unable to parse value. COLOR must be in the format #RRGGBB",
        clap::ErrorKind::InvalidValue,
    )
}

fn arg_to_color(matches: &ArgMatches, arg_name: &str) -> Result<graphics::Color, clap::Error> {
    let value = match matches.value_of(arg_name) {
        Some(v) => v,
        None => return Err(get_invalid_color_error()),
    };
    to_color(value)
}

fn to_color(hex_color: &str) -> Result<graphics::Color, clap::Error> {
    let hex_rgb;
    if hex_color.chars().nth(0).unwrap() == '#' {
        hex_rgb = &hex_color[1..];
    } else {
        hex_rgb = hex_color;
    }
    if hex_rgb.len() != 6 {
        return Err(get_invalid_color_error());
    }
    match u32::from_str_radix(&hex_rgb, 16) {
        Ok(v) => return Ok(graphics::Color::from_rgb_u32(v)),
        Err(_) => return Err(get_invalid_color_error()),
    }
}
