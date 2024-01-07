//! A trivial example for a multiplier function

use rust_hdl::prelude::*;

#[rust_hls_macro::hls]
pub mod multiplier {
    #[hls(include_logs, include_llvm_ir)]
    pub extern "C" fn multiplier(a: u32, b: u32) -> u32 {
        b * a
    }
}

pub fn main() {
    let mut device: Multiplier = Multiplier::new();
    device.connect_all();
    let data = generate_verilog(&device);
    std::fs::write("./multiplier.v", data).unwrap();
}

#[cfg(test)]
mod tests {
    use super::Multiplier;
    use rust_hdl::prelude::*;

    #[test]
    fn multiplier_works() {
        let mut multiplier = Multiplier::default();
        multiplier.connect_all();

        let mut simulation = Simulation::new();
        simulation.add_clock(1, |counter: &mut Box<Multiplier>| {
            counter.clk.next = !counter.clk.val()
        });

        simulation.add_testbench(move |mut sim: Sim<Multiplier>| {
            let mut multiplier = sim.init()?;

            multiplier.a.next = 4u32.to_bits();
            multiplier.b.next = 21u32.to_bits();

            wait_clock_cycle!(sim, clk, multiplier);

            let result: u32 = multiplier.return_port.val().to_u32();
            sim_assert_eq!(sim, result, 84, multiplier);

            sim.done(multiplier)
        });

        simulation.run(Box::new(multiplier), 20).unwrap();
    }

    // // TODO: For now only one HLS function per crate is supported
    // #[test]
    // fn other_function_works() {
    //     let mut multiplier = YourFunction::default();
    //     multiplier.connect_all();

    //     let mut simulation = Simulation::new();
    //     simulation.add_clock(1, |counter: &mut Box<YourFunction>| {
    //         counter.clk.next = !counter.clk.val()
    //     });

    //     simulation.add_testbench(move |mut sim: Sim<YourFunction>| {
    //         let mut multiplier = sim.init()?;

    //         multiplier.a.next = 4u32.to_bits();
    //         multiplier.b.next = 20u32.to_bits();

    //         wait_clock_cycle!(sim, clk, multiplier);

    //         sim_assert_eq!(sim, multiplier.return_port.val(), 76, multiplier);

    //         sim.done(multiplier)
    //     });

    //     simulation.run(Box::new(multiplier), 20).unwrap();
    // }
}
