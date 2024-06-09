use std::env;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::process;

mod display;

const ELF_HEADER_SIZE: usize = 64;
const ELF_MAGIC: [u8; 4] = [0x7F, 0x45, 0x4C, 0x46];
const PHDR_ENTRY_SIZE: usize = 56;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the program name is provided
    if args.len() != 2 {
        println!("Usage: {} <program_name>", args[0]);
        return;
    }

    // Open the file
    let mut file = get_file();

    // Read the ELF header
    let mut header = [0u8; ELF_HEADER_SIZE];
    if let Err(e) = file.read_exact(&mut header) {
        println!("Failed to read the ELF header: {}", e);
        return;
    }

    let magic = &header[..4];
    let magic_string = magic
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<String>();

    // Check if the file is an ELF file
    if magic != ELF_MAGIC {
        println!("{} is not an ELF file ({})", args[1], magic_string);
        return;
    }

    // Extract the ELF file type
    let (elf_type_str, elf_type) = get_elf_type(&header);

    let (machine_type_str, machine_type) = get_machine_type(&header);

    // Extract the ELF class
    let (elf_class_str, _) = get_elf_class(&header);

    // Extract the data encoding
    let (data_encoding_str, _) = get_data_encoding(&header);

    // Extract the ELF version
    let elf_version = get_elf_version(&header);

    // Extract the entry point address
    let entry_point = get_entry_point(&header);

    // Extract the program header table offset
    let phdr_offset = get_phdr_offset(&header);

    // Extract the section header table offset
    let shdr_offset = get_shdr_offset(&header);

    // Write a function that takes two bytes and returns a u16
    // Extract the ELF header size
    let elf_header_size = bits_to_u16(&[header[0x34], header[0x35]]);

    // Extract the program header table entry size
    let phdr_entry_size = bits_to_u16(&[header[0x36], header[0x37]]);

    // Extract the number of program header table entries
    let phdr_num_entries = bits_to_u16(&[header[0x38], header[0x39]]);

    // Extract the section header table entry size
    let shdr_entry_size = bits_to_u16(&[header[0x3A], header[0x3B]]);

    // Extract the number of section header table entries
    let shdr_num_entries = bits_to_u16(&[header[0x3C], header[0x3D]]);

    // Extract the section header string table index
    let shdr_str_index = bits_to_u16(&[header[0x3E], header[0x3F]]);

    // Print the extracted information
    println!("Full header:");
    for byte in &header[..ELF_HEADER_SIZE] {
        print!("{:02X} ", byte);
    }
    println!("ELF File Type: {} (0x{:02X})", elf_type_str, elf_type);
    println!(
        "Machine Type: {} (0x{:04X})",
        machine_type_str, machine_type
    );
    println!();
    println!("ELF Class: {}", elf_class_str);
    println!("Data Encoding: {}", data_encoding_str);
    println!("ELF Version: {}", elf_version);
    println!("Entry Point Address: {}", entry_point);
    println!("Program Header Table Offset: {}", phdr_offset);
    println!("Section Header Table Offset: {}", shdr_offset);
    println!("ELF Header Size: {} bytes", elf_header_size);
    println!("Program Header Table Entry Size: {} bytes", phdr_entry_size);
    println!(
        "Number of Program Header Table Entries: {}",
        phdr_num_entries
    );
    println!("Section Header Table Entry Size: {} bytes", shdr_entry_size);
    println!(
        "Number of Section Header Table Entries: {}",
        shdr_num_entries
    );
    println!("Section Header String Table Index: {}", shdr_str_index);

    // Extract the number of program header table entries
    let phdr_num_entries = bits_to_u16(&[header[0x38], header[0x39]]);

    // Seek to the program header table offset
    file.seek(SeekFrom::Start(phdr_offset)).unwrap();

    // Print the segment information
    println!("\nSegment Information:");
    for i in 0..phdr_num_entries {
        let mut phdr_entry = [0u8; PHDR_ENTRY_SIZE];
        file.read_exact(&mut phdr_entry).unwrap();

        let segment_type = get_phdr_segment_type(&phdr_entry);
        let segment_offset = get_phdr_segment_offset(&phdr_entry);
        let segment_vaddr = get_phdr_segment_vaddr(&phdr_entry);
        let segment_paddr = get_phdr_segment_paddr(&phdr_entry);
        let segment_filesz = get_phdr_segment_filesz(&phdr_entry);
        let segment_memsz = get_phdr_segment_memsz(&phdr_entry);
        let segment_flags = get_phdr_segment_flags(&phdr_entry);

        println!("Segment {}:", i);
        println!(
            "  Type: {} (0x{:08X})",
            get_segment_type(segment_type),
            segment_type
        );
        println!("  Offset: {}", segment_offset);
        println!("  Virtual Address: {}", segment_vaddr);
        println!("  Physical Address: {}", segment_paddr);
        println!("  File Size: {}", segment_filesz);
        println!("  Memory Size: {}", segment_memsz);
        println!(
            "  Flags: {} (0x{:08X})",
            get_segment_flags(segment_flags),
            segment_flags
        );
        println!();
    }
}

fn get_entry_point(header: &[u8]) -> u64 {
    u64::from_le_bytes([
        header[0x18],
        header[0x19],
        header[0x1A],
        header[0x1B],
        header[0x1C],
        header[0x1D],
        header[0x1E],
        header[0x1F],
    ])
}

fn get_phdr_offset(header: &[u8]) -> u64 {
    u64::from_le_bytes([
        header[0x20],
        header[0x21],
        header[0x22],
        header[0x23],
        header[0x24],
        header[0x25],
        header[0x26],
        header[0x27],
    ])
}

fn get_shdr_offset(header: &[u8]) -> u64 {
    u64::from_le_bytes([
        header[0x28],
        header[0x29],
        header[0x2A],
        header[0x2B],
        header[0x2C],
        header[0x2D],
        header[0x2E],
        header[0x2F],
    ])
}

fn get_phdr_segment_type(phdr_entry: &[u8]) -> u32 {
    u32::from_le_bytes([
        phdr_entry[0x04],
        phdr_entry[0x05],
        phdr_entry[0x06],
        phdr_entry[0x07],
    ])
}

fn get_phdr_segment_offset(phdr_entry: &[u8]) -> u64 {
    u64::from_le_bytes([
        phdr_entry[0x08],
        phdr_entry[0x09],
        phdr_entry[0x0A],
        phdr_entry[0x0B],
        phdr_entry[0x0C],
        phdr_entry[0x0D],
        phdr_entry[0x0E],
        phdr_entry[0x0F],
    ])
}

fn get_phdr_segment_vaddr(phdr_entry: &[u8]) -> u64 {
    u64::from_le_bytes([
        phdr_entry[0x10],
        phdr_entry[0x11],
        phdr_entry[0x12],
        phdr_entry[0x13],
        phdr_entry[0x14],
        phdr_entry[0x15],
        phdr_entry[0x16],
        phdr_entry[0x17],
    ])
}

fn get_phdr_segment_paddr(phdr_entry: &[u8]) -> u64 {
    u64::from_le_bytes([
        phdr_entry[0x18],
        phdr_entry[0x19],
        phdr_entry[0x1A],
        phdr_entry[0x1B],
        phdr_entry[0x1C],
        phdr_entry[0x1D],
        phdr_entry[0x1E],
        phdr_entry[0x1F],
    ])
}

fn get_phdr_segment_filesz(phdr_entry: &[u8]) -> display::HexAddress {
    display::HexAddress(u64::from_le_bytes([
        phdr_entry[0x20],
        phdr_entry[0x21],
        phdr_entry[0x22],
        phdr_entry[0x23],
        phdr_entry[0x24],
        phdr_entry[0x25],
        phdr_entry[0x26],
        phdr_entry[0x27],
    ]))
}

fn get_phdr_segment_memsz(phdr_entry: &[u8]) -> display::HexAddress {
    display::HexAddress(u64::from_le_bytes([
        phdr_entry[0x28],
        phdr_entry[0x29],
        phdr_entry[0x2A],
        phdr_entry[0x2B],
        phdr_entry[0x2C],
        phdr_entry[0x2D],
        phdr_entry[0x2E],
        phdr_entry[0x2F],
    ]))
}

fn get_phdr_segment_flags(phdr_entry: &[u8]) -> u32 {
    u32::from_le_bytes([
        phdr_entry[0x30],
        phdr_entry[0x31],
        phdr_entry[0x32],
        phdr_entry[0x33],
    ])
}

fn get_file() -> File {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        process::exit(1);
    }

    let file = File::open(&args[1]);
    if file.is_err() {
        eprintln!("Failed to open file: {}", args[1]);
        process::exit(1);
    }

    file.unwrap()
}

fn get_segment_type(segment_type: u32) -> &'static str {
    match segment_type {
        0x00000000 => "PT_NULL",
        0x00000001 => "PT_LOAD",
        0x00000002 => "PT_DYNAMIC",
        0x00000003 => "PT_INTERP",
        0x00000004 => "PT_NOTE",
        0x00000005 => "PT_SHLIB",
        0x00000006 => "PT_PHDR",
        0x00000007 => "PT_TLS",
        0x6474e550 => "PT_GNU_EH_FRAME",
        0x6474e551 => "PT_GNU_STACK",
        0x6474e552 => "PT_GNU_RELRO",
        _ => "Unknown",
    }
}

fn get_segment_flags(segment_flags: u32) -> String {
    let mut flags = String::new();
    match segment_flags {
        0x01 => flags.push_str("R"),
        0x02 => flags.push_str("W"),
        0x04 => flags.push_str("X"),
        _ => flags.push_str("Unknown"),
    }
    flags
}

fn get_machine_type(header: &[u8]) -> (String, u16) {
    let machine_type = u16::from_le_bytes([header[0x12], header[0x13]]);
    let machine_type_str = match machine_type {
        0x03 => "x86",
        0x3E => "x86-64",
        0xB7 => "AArch64",
        _ => "Unknown",
    }
    .to_string();
    (machine_type_str, machine_type)
}

fn get_elf_type(header: &[u8]) -> (String, u8) {
    let elf_type = header[0x10];
    let elf_type_str = match elf_type {
        0x01 => "Relocatable",
        0x02 => "Executable",
        0x03 => "Shared",
        0x04 => "Core",
        _ => "Unknown",
    }
    .to_string();
    (elf_type_str, elf_type)
}

fn get_elf_class(header: &[u8]) -> (String, u8) {
    let elf_class = header[0x04];
    let elf_class_str = match elf_class {
        1 => "32-bit",
        2 => "64-bit",
        _ => "Unknown",
    }
    .to_string();
    (elf_class_str, elf_class)
}

fn get_data_encoding(header: &[u8]) -> (String, u8) {
    let data_encoding = header[0x05];
    let data_encoding_str = match data_encoding {
        1 => "Little-endian",
        2 => "Big-endian",
        _ => "Unknown",
    }
    .to_string();
    (data_encoding_str, data_encoding)
}

fn bits_to_u16(bits: &[u8; 2]) -> u16 {
    let value = u16::from_le_bytes(*bits);
    value
}

fn get_elf_version(header: &[u8]) -> u8 {
    let elf_version = header[0x06];
    elf_version
}
