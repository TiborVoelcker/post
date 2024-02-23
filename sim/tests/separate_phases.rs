// Created by Tibor Völcker (tiborvoelcker@hotmail.de) on 11.02.24
// Last modified by Tibor Völcker on 19.02.24
// Copyright (c) 2024 Tibor Völcker (tiborvoelcker@hotmail.de)

use sim::example_data::example_data;
use sim::*;
use utils::assert_almost_eq_rel;

#[test]
fn phase_1() {
    let data = example_data();

    let mut planet = EARTH_SPHERICAL;
    planet.add_atmosphere(Atmosphere::StandardAtmosphere1962);
    let vehicle = data[0].vehicle.clone();

    let mut sim = Simulation::new(vehicle, planet, 5.);
    sim.init_geodetic(28.5, 279.4, 90.);
    sim.state.mass = data[0].mass;

    assert_eq!(sim.state.time, data[0].time);
    assert_almost_eq_rel!(sim.state.mass, data[0].mass, 0.001);
    assert_almost_eq_rel!(sim.state.position[0], data[0].position[0], 0.001);
    assert_almost_eq_rel!(sim.state.position[1], data[0].position[1], 0.001);
    assert_almost_eq_rel!(sim.state.position[2], data[0].position[2], 0.001);
    assert_almost_eq_rel!(sim.state.velocity[0], data[0].velocity[0], 0.001);
    assert_almost_eq_rel!(sim.state.velocity[1], data[0].velocity[1], 0.001);
    assert_almost_eq_rel!(sim.state.velocity[2], data[0].velocity[2], 0.001);

    println!(
        "Time: {:.0}\nPosition: {:.0}\nVelocity: {:.0}",
        sim.state.time, sim.state.position, sim.state.velocity
    );
    while sim.state.time < data[1].time {
        sim.step();
        println!(
            "Time: {:.0}\nPosition: {:.0}\nVelocity: {:.0}",
            sim.state.time, sim.state.position, sim.state.velocity
        );
    }
    assert_eq!(sim.state.time, data[1].time);
    assert_almost_eq_rel!(sim.state.mass, data[1].mass, 0.001);
    assert_almost_eq_rel!(sim.state.position[0], data[1].position[0], 0.001);
    assert_almost_eq_rel!(sim.state.position[1], data[1].position[1], 0.001);
    assert_almost_eq_rel!(sim.state.position[2], data[1].position[2], 0.001);
    assert_almost_eq_rel!(sim.state.velocity[0], data[1].velocity[0], 0.001);
    assert_almost_eq_rel!(sim.state.velocity[1], data[1].velocity[1], 0.001);
    assert_almost_eq_rel!(sim.state.velocity[2], data[1].velocity[2], 0.001);
}

#[test]
fn phase_11() {
    let data = example_data();

    let mut planet = EARTH_SPHERICAL;
    planet.add_atmosphere(Atmosphere::StandardAtmosphere1962);
    let vehicle = data[2].vehicle.clone();

    let mut sim = Simulation::new(vehicle, planet, data[3].time);
    sim.init_geodetic(28.5, 279.4, 90.);
    sim.init_inertial(data[2].position, data[2].velocity);
    sim.state.mass = data[2].mass;

    println!(
        "Time: {:.0}\nPosition: {:.0}\nVelocity: {:.0}",
        sim.state.time, sim.state.position, sim.state.velocity
    );
    while sim.state.time < 20. {
        sim.step();
        println!(
            "Time: {:.0}\nPosition: {:.0}\nVelocity: {:.0}",
            sim.state.time, sim.state.position, sim.state.velocity
        );
    }
    assert_almost_eq_rel!(sim.state.time, data[3].time, 0.001);
    assert_almost_eq_rel!(sim.state.mass, data[3].mass, 0.001);
    assert_almost_eq_rel!(sim.state.position[0], data[3].position[0], 0.001);
    assert_almost_eq_rel!(sim.state.position[1], data[3].position[1], 0.001);
    assert_almost_eq_rel!(sim.state.position[2], data[3].position[2], 0.001);
    assert_almost_eq_rel!(sim.state.velocity[0], data[3].velocity[0], 0.001);
    assert_almost_eq_rel!(sim.state.velocity[1], data[3].velocity[1], 0.001);
    assert_almost_eq_rel!(sim.state.velocity[2], data[3].velocity[2], 0.001);
}