// Created by Tibor Völcker (tiborvoelcker@hotmail.de) on 22.11.23
// Last modified by Tibor Völcker on 22.05.24
// Copyright (c) 2023 Tibor Völcker (tiborvoelcker@hotmail.de)

//! Constants used throughout the crate. They are either from [3, p. IV-7] or
//! from https://physics.nist.gov/cuu/pdf/sp811.pdf p. 57-69 ff.

pub const NEARLY_ZERO: f64 = 1e-6;

// CONVERSIONS from https://physics.nist.gov/cuu/pdf/sp811.pdf p. 57-69 ff.
pub const METER_PER_FOOT: f64 = 3.048e-01;
pub const SQUARE_METER_PER_SQUARE_FOOT: f64 = METER_PER_FOOT * METER_PER_FOOT;
pub const CUBIC_METER_PER_CUBIC_FOOT: f64 = METER_PER_FOOT * METER_PER_FOOT * METER_PER_FOOT;
pub const KELVIN_PER_RANKIN: f64 = 1. / 1.8;
pub const KILOGRAM_PER_POUND: f64 = 4.535924e-1;
pub const PASCAL_PER_PSF: f64 = KILOGRAM_PER_POUND * STD_GRAVITY / SQUARE_METER_PER_SQUARE_FOOT;

#[cfg(test)]
pub const KILOGRAM_PER_SLUG: f64 = 1.459390e+01;
#[cfg(test)]
pub const NEWTON_PER_POUND_FORCE: f64 = KILOGRAM_PER_POUND * STD_GRAVITY;

// CONSTANTS from [3] p. IV-7
pub const STD_GRAVITY: f64 = 9.80665; // [m / s^2]
const AIR_MOLECULAR_WEIGHT: f64 = 28.9644; // [g / mol]
const GAS_CONSTANT: f64 = 8.31432e3; // [J / kmol K]
pub const AIR_KAPPA: f64 = 1.40; // [-]
pub const AIR_GAS_CONSTANT: f64 = GAS_CONSTANT / AIR_MOLECULAR_WEIGHT; // [J / kg K]
