use cyclotron::sim::Sim;

fn main() {
    let mut sim = Sim::new();

    for _ in 0..10 {
        let imem_req = sim.imem_req.get();
        match imem_req {
            Some(data) => {
                println!("detected imem req: data={}", data);
                assert!(
                    sim.imem_resp.put(data + 42, sim.time() + 1),
                    "response put failed!"
                ); // bogus
            }
            None => {}
        }

        sim.tick();
    }
}
