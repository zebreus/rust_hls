//! A trivial example for a adder function

use rust_hdl::prelude::*;

#[rust_hls_macro::hls]
pub mod adder {
    #[hls(include_logs, include_llvm_ir)]
    pub extern "C" fn adder(a: u32, b: u32) -> u32 {
        return b + a;
    }
}

pub fn main() {
    let mut device: Adder = Adder::new();
    device.connect_all();
    let data = generate_verilog(&device);
    std::fs::write("./adder.v", data).unwrap();
}

#[cfg(test)]
mod tests {
    use super::Adder;
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

            adder.a.next = 65u32.to_bits();
            adder.b.next = 47u32.to_bits();

            wait_clock_cycle!(sim, clk, adder);

            let result: u32 = adder.return_port.val().to_u32();
            sim_assert_eq!(sim, result, 112, adder);

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
