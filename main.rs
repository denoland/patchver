use libsui::Elf;
use libsui::Macho;
use libsui::PortableExecutable;

use libsui::utils;

const HELP: &str = r#"Usage: patchver <exe> <data_file> <output>"#;
const SECTION: &str = "denoversion";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("{}", HELP);
        std::process::exit(1);
    }

    let exe = std::fs::read(&args[1])?;
    let data = std::fs::read(&args[2])?;

    let mut out = std::fs::File::create(&args[3])?;

    if utils::is_pe(&exe) {
        PortableExecutable::from(&exe)?
            .write_resource(SECTION, data)?
            .build(&mut out)?;
    } else if utils::is_macho(&exe) {
        Macho::from(exe)?
            .write_section(SECTION, data)?
            .build_and_sign(&mut out)?;
    } else if utils::is_elf(&exe) {
        Elf::new(&exe).append(SECTION, &data, &mut out)?;
    } else {
        eprintln!("Unsupported file format");
        std::process::exit(1);
    }

    Ok(())
}
