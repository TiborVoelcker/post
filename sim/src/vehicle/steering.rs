// Created by Tibor Völcker (tiborvoelcker@hotmail.de) on 06.12.23
// Last modified by Tibor Völcker on 22.12.23
// Copyright (c) 2023 Tibor Völcker (tiborvoelcker@hotmail.de)

use nalgebra::Vector4;

pub enum Steering {
    // Rate,
    Angular(Angular),
}

pub enum Angular {
    Polynomials(Vector4<f64>),
    // Tables,
    // LinearEquations,
    // ClosedLoop,
}

impl Steering {
    pub fn update(&self, variable: f64) -> f64 {
        match self {
            Steering::Angular(steering_type) => match steering_type {
                Angular::Polynomials(coeffs) => {
                    // See [1] p. V-22
                    Vector4::from_iterator((0..4).map(|i| variable.powi(i)))
                        .component_mul(coeffs)
                        .sum()
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::vector;

    use super::*;

    #[test]
    fn angular_polynomials() {
        let steering = Steering::Angular(Angular::Polynomials(vector![4., 3., 2., 1.]));

        assert_eq!(
            steering.update(2.),
            4. + 3. * 2. + 2. * 2_f64.powi(2) + 1. * 2_f64.powi(3)
        )
    }
}
