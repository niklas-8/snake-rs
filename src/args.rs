extern crate clap;
extern crate ggez;

use crate::options::Options;
use clap::{App, Arg};
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

    let grid_width;
    match to_i16(matches.value_of("grid-width").unwrap()) {
        Ok(v) => grid_width = v,
        Err(e) => return Err(e),
    }

    let grid_height;
    match to_i16(matches.value_of("grid-height").unwrap()) {
        Ok(v) => grid_height = v,
        Err(e) => return Err(e),
    }

    let grid_cell_width;
    match to_i16(matches.value_of("grid-cell-width").unwrap()) {
        Ok(v) => grid_cell_width = v,
        Err(e) => return Err(e),
    }

    let grid_cell_height;
    match to_i16(matches.value_of("grid-cell-height").unwrap()) {
        Ok(v) => grid_cell_height = v,
        Err(e) => return Err(e),
    }

    let background_color;
    match to_color(matches.value_of("background-color").unwrap()) {
        Ok(v) => background_color = v,
        Err(e) => return Err(e),
    }

    let snake_head_color;
    match to_color(matches.value_of("snake-head-color").unwrap()) {
        Ok(v) => snake_head_color = v,
        Err(e) => return Err(e),
    }

    let snake_body_color;
    match to_color(matches.value_of("snake-body-color").unwrap()) {
        Ok(v) => snake_body_color = v,
        Err(e) => return Err(e),
    }

    let food_color;
    match to_color(matches.value_of("food-color").unwrap()) {
        Ok(v) => food_color = v,
        Err(e) => return Err(e),
    }

    let millis_per_update;
    match to_i16(matches.value_of("updates-per-second").unwrap()) {
        Ok(v) => millis_per_update = (1.0 / v as f32 * 1000.0) as u64,
        Err(e) => return Err(e),
    }

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

fn to_i16(value: &str) -> Result<i16, clap::Error> {
    match value.parse::<i16>() {
        Ok(v) => return Ok(v),
        Err(_) => {
            return Err(clap::Error::with_description(
                "Unable to parse value. NUMBER must be an integer with 16 bits.",
                clap::ErrorKind::InvalidValue,
            ))
        }
    }
}

fn to_color(hex_color: &str) -> Result<graphics::Color, clap::Error> {
    let hex_rgb;
    if hex_color.chars().nth(0).unwrap() == '#' {
        hex_rgb = &hex_color[1..];
    } else {
        hex_rgb = hex_color;
    }
    match u32::from_str_radix(&hex_rgb, 16) {
        Ok(v) => return Ok(graphics::Color::from_rgb_u32(v)),
        Err(_) => {
            return Err(clap::Error::with_description(
                "Unable to parse value. COLOR must be in the format #RRGGBB",
                clap::ErrorKind::InvalidValue,
            ))
        }
    }
}
