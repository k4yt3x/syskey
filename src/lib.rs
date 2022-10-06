use std::{fs::File, io::Write, path::PathBuf};

use anyhow::Result;
use phf::phf_map;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

static V: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "1",
    "1" => "0",
    "2" => "7",
    "3" => "6",
    "4" => "5",
    "5" => "4",
    "6" => "B",
    "7" => "A",
    "8" => "9",
    "9" => "8",
    "A" => "F",
    "B" => "E",
    "C" => "D",
    "D" => "C",
    "E" => "3",
    "F" => "2",
};

static W: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "A",
    "1" => "9",
    "2" => "8",
    "3" => "7",
    "4" => "6",
    "5" => "5",
    "6" => "4",
    "7" => "3",
    "8" => "2",
    "9" => "1",
    "A" => "0",
};

static Y: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "A",
    "1" => "B",
    "2" => "8",
    "3" => "9",
    "4" => "E",
    "5" => "F",
    "6" => "6",
    "7" => "D",
    "8" => "2",
    "9" => "3",
    "A" => "0",
    "B" => "1",
    "C" => "6",
    "D" => "7",
    "E" => "4",
    "F" => "5",
};

static Z: phf::Map<&'static str, &'static str> = phf_map! {
    "0" => "1",
    "1" => "0",
    "2" => "3",
    "3" => "2",
    "4" => "5",
    "5" => "4",
    "6" => "7",
    "7" => "6",
    "8" => "9",
    "9" => "8",
    "A" => "B",
    "B" => "A",
    "C" => "D",
    "D" => "C",
    "E" => "F",
    "F" => "E",
};

static VW: phf::Map<&'static str, &'static str> = phf_map! {
    "0B" => "0F",
    "0C" => "0E",
    "0D" => "0D",
    "0E" => "0C",
    "0F" => "0B",
    "1B" => "7F",
    "1C" => "7E",
    "1D" => "7D",
    "1E" => "7C",
    "1F" => "7B",
    "2B" => "6F",
    "2C" => "6E",
    "2D" => "6D",
    "2E" => "6C",
    "2F" => "6B",
    "3B" => "5F",
    "3C" => "5E",
    "3D" => "5C",
    "3E" => "5C",
    "3F" => "5B",
    "4B" => "4F",
    "4C" => "4E",
    "4D" => "4C",
    "4E" => "4C",
    "4F" => "4B",
    "5B" => "BF",
    "5C" => "BE",
    "5D" => "BC",
    "5E" => "BC",
    "5F" => "BB",
    "6B" => "AF",
    "6C" => "AE",
    "6D" => "AD",
    "6E" => "AC",
    "6F" => "AB",
    "7B" => "9F",
    "7C" => "9E",
    "7D" => "9D",
    "7E" => "9C",
    "7F" => "9B",
    "8B" => "8F",
    "8C" => "8E",
    "8D" => "8D",
    "8E" => "8C",
    "8F" => "8B",
    "9B" => "FF",
    "9C" => "FE",
    "9D" => "FD",
    "9E" => "FC",
    "9F" => "FB",
    "AB" => "EF",
    "AC" => "EE",
    "AD" => "ED",
    "AE" => "EC",
    "AF" => "EB",
    "BB" => "DF",
    "BC" => "DE",
    "BD" => "DD",
    "BE" => "DC",
    "BF" => "DB",
    "CB" => "CF",
    "CC" => "CE",
    "CD" => "CD",
    "CE" => "CC",
    "CF" => "CB",
    "DB" => "3F",
    "DC" => "3E",
    "DD" => "3D",
    "DE" => "3C",
    "DF" => "3B",
    "EB" => "2F",
    "EC" => "2E",
    "ED" => "2D",
    "EE" => "2C",
    "EF" => "2B",
    "FB" => "1F",
    "FC" => "1E",
    "FD" => "1D",
    "FE" => "1C",
    "FF" => "1B",
};

fn create_syskey(sysid: &str) -> Result<Vec<u8>> {
    let chars: Vec<&str> = sysid.split("").filter(|&x| !x.is_empty()).collect();
    let vwkey = format!("{}{}", chars[0], chars[1]);
    let vw = {
        if VW.contains_key(&vwkey) {
            hex::decode(VW[&vwkey])?
        }
        else {
            hex::decode(format!("{}{}", V[chars[0]], W[chars[1]]))?
        }
    };
    let yz = hex::decode(format!("{}{}", Y[chars[2]], Z[chars[3]]))?;
    eprintln!("debug: generated vw={:0>2x}, yz={:0>2x}", vw[0], yz[0]);

    Ok(vec![
        0x2C, 0xB5, 0x32, 0xB3, 0x88, 0x0D, 0x6A, 0xDF, 0x4B, 0xB4, 0xE9, 0x95, 0x2C, 0x6D, 0x1C,
        0x03, 0x37, 0x24, 0x1F, 0x5B, 0x93, 0xBF, 0x24, 0x1A, vw[0], 0xFF, yz[0],
    ])
}

pub fn run(sysid: String) -> Result<()> {
    let syskey_bytes = create_syskey(&sysid)?;
    let mut output = File::create(PathBuf::from(format!("{}.key", &sysid)))?;
    output.write_all(&syskey_bytes)?;
    Ok(())
}
