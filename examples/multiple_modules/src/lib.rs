//! A trivial example for a adder function

use rust_hdl::prelude::*;

#[rust_hls_macro::hls]
pub mod divider {

    #[hls(include_logs, include_llvm_ir)]
    pub extern "C" fn divider(a: u32, mut b: u32) -> u32 {
        if b == 0 {
            b = 2;
        }
        a / b
    }
}

#[rust_hls_macro::hls]
pub mod subtractor {
    #[hls(include_logs, include_llvm_ir)]
    pub extern "C" fn subtractor(a: u32, b: u32) -> u32 {
        a - b
    }
}

pub fn main() {
    let mut device: Divider = Divider::new();
    device.connect_all();
    let data = generate_verilog(&device);
    std::fs::write("./adder.v", data).unwrap();

    let mut device: Subtractor = Subtractor::new();
    device.connect_all();
    let data = generate_verilog(&device);
    std::fs::write("./subtractor.v", data).unwrap();
}

#[cfg(test)]
mod tests {
    use super::Divider;
    use super::Subtractor;
    use rust_hdl::prelude::*;

    #[test]
    fn subtractor_works() {
        let mut subtractor = Subtractor::default();
        subtractor.connect_all();

        let mut simulation = Simulation::new();
        simulation.add_clock(1, |counter: &mut Box<Subtractor>| {
            counter.clk.next = !counter.clk.val()
        });

        simulation.add_testbench(move |mut sim: Sim<Subtractor>| {
            let mut subtractor = sim.init()?;

            subtractor.a.next = 65u32.to_bits();
            subtractor.b.next = 22u32.to_bits();

            wait_clock_cycle!(sim, clk, subtractor);

            let result: u32 = subtractor.return_port.val().to_u32();
            sim_assert_eq!(sim, result, 43, subtractor);

            sim.done(subtractor)
        });

        simulation.run(Box::new(subtractor), 20).unwrap();
    }

    #[test]
    fn divider_works() {
        let mut divider = Divider::default();
        divider.connect_all();

        let mut simulation: Simulation<Divider> = Simulation::new();
        simulation.add_clock(1, |counter: &mut Box<Divider>| {
            counter.clk.next = !counter.clk.val()
        });

        simulation.add_testbench(move |mut sim: Sim<Divider>| {
            let mut divider = sim.init()?;

            divider.a.next = 20000u32.to_bits();
            divider.b.next = 9999u32.to_bits();

            wait_clock_cycle!(sim, clk, divider);

            let result: u32 = divider.return_port.val().to_u32();
            sim_assert_eq!(sim, result, 18, divider);

            sim.done(divider)
        });

        simulation.run(Box::new(divider), 100).unwrap();
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
