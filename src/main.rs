use rustboy::gameboy::GameBoy;

fn main() -> std::io::Result<()> {
    env_logger::init();
    let filename = std::env::args().last().unwrap();
    let mut gb = GameBoy::load_cartridge(&filename)?;
    gb.run();
    Ok(())
}
