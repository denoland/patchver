use libsui::Elf;
use libsui::Macho;
use libsui::PortableExecutable;

use libsui::utils;

const CHANNELS: [&str; 4] = ["stable", "lts", "canary", "rc"];
const SECTION: &str = "denover";

pub fn patchver<W: std::io::Write>(
    exe: Vec<u8>,
    data: String,
    out: &mut W,
) -> Result<(), Box<dyn std::error::Error>> {
    if !CHANNELS.contains(&data.as_str()) {
        panic!("Invalid channel. Expected one of: stable, lts, canary, rc");
    }

    let data = data.as_bytes().to_vec();
    if utils::is_pe(&exe) {
        PortableExecutable::from(&exe)?
            .write_resource(SECTION, data)?
            .build(out)?;
    } else if utils::is_macho(&exe) {
        Macho::from(exe)?
            .write_section(SECTION, data)?
            .build_and_sign(out)?;
    } else if utils::is_elf(&exe) {
        Elf::new(&exe).append(SECTION, &data, out)?;
    } else {
        panic!("Unsupported file format");
    }

    Ok(())
}
