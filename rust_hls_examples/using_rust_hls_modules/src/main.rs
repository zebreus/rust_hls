//! This example demonstrates how crates that use rust-hls can be used in another crate.
//!
//! In this case we use the adder and the multiplier examples

use adder::Adder;
use multiplier::Multiplier;
use rust_hdl::prelude::*;

pub fn main() {
    let mut device: Adder = Adder::new();
    device.connect_all();
    let data = generate_verilog(&device);
    std::fs::write("./adder.v", data).unwrap();

    let mut device: Multiplier = Multiplier::new();
    device.connect_all();
    let data = generate_verilog(&device);
    std::fs::write("./multiplier.v", data).unwrap();
}

#[cfg(test)]
mod tests {
    use adder::Adder;
    use multiplier::Multiplier;

    use rust_hdl::prelude::*;

    #[test]
    fn adder_works() {
        let mut adder = Adder::default();
        adder.connect_all();

        let mut simulation = Simulation::new();
        simulation.add_clock(1, |counter: &mut Box<Adder>| {
            counter.clk.next = !counter.clk.val()
        });

        simulation.add_testbench(move |mut sim: Sim<Adder>| {
            let mut adder = sim.init()?;

            adder.a.next = 4u32.to_bits();
            adder.b.next = 20u32.to_bits();

            wait_clock_cycle!(sim, clk, adder);

            let result: u32 = adder.return_port.val().to_u32();
            sim_assert_eq!(sim, result, 24, adder);

            sim.done(adder)
        });

        simulation.run(Box::new(adder), 20).unwrap();
    }

    #[test]
    fn multiplier_works() {
        let mut adder = Multiplier::default();
        adder.connect_all();

        let mut simulation = Simulation::new();
        simulation.add_clock(1, |counter: &mut Box<Multiplier>| {
            counter.clk.next = !counter.clk.val()
        });

        simulation.add_testbench(move |mut sim: Sim<Multiplier>| {
            let mut adder = sim.init()?;

            adder.a.next = 4u32.to_bits();
            adder.b.next = 20u32.to_bits();

            wait_clock_cycle!(sim, clk, adder);

            let result: u32 = adder.return_port.val().to_u32();
            sim_assert_eq!(sim, result, 80, adder);

            sim.done(adder)
        });

        simulation.run(Box::new(adder), 20).unwrap();
    }

    // // TODO: For now only one HLS function per crate is supported
    // #[test]
    // fn other_function_works() {
    //     let mut adder = YourFunction::default();
    //     adder.connect_all();

    //     let mut simulation = Simulation::new();
    //     simulation.add_clock(1, |counter: &mut Box<YourFunction>| {
    //         counter.clk.next = !counter.clk.val()
    //     });

    //     simulation.add_testbench(move |mut sim: Sim<YourFunction>| {
    //         let mut adder = sim.init()?;

    //         adder.a.next = 4u32.to_bits();
    //         adder.b.next = 20u32.to_bits();

    //         wait_clock_cycle!(sim, clk, adder);

    //         sim_assert_eq!(sim, adder.return_port.val(), 76, adder);

    //         sim.done(adder)
    //     });

    //     simulation.run(Box::new(adder), 20).unwrap();
    // }
}
