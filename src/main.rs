use std::env;
use std::fs::{read_dir, File};
use std::io::{Read, Result};
use std::collections::HashMap;
use std::process::Command;
use indicatif::ProgressBar;
use iced_x86::{
    Decoder, DecoderOptions, Instruction,
};

#[allow(dead_code)]
pub(crate) fn decode_codes(data: &[u8], map: &mut HashMap<String, i32>) {
    let mut decoder = Decoder::with_ip(64, data, 0x0, DecoderOptions::NONE);
    let mut instr = Instruction::default();
    while decoder.can_decode() {
        decoder.decode_out(&mut instr);
        map.entry(format!("{:?}", instr.code())).and_modify(|count| *count += 1).or_insert(1);
    }
}

fn find_object_files(dir: &str) -> Result<Vec<String>> {
    let mut files = Vec::new();

    for entry in read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".o") {
                    files.push(entry.path().display().to_string());
                }
            }
        } else if entry.file_type()?.is_dir() {
            let sub_dir = entry.path().display().to_string();
            files.extend(find_object_files(&sub_dir)?);
        }
    }

    Ok(files)
}

fn main() {
    let mut counted_codes: HashMap<String, i32> = HashMap::new();
    let args: Vec<String> = env::args().collect();
    let dir = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };

    match find_object_files(dir) {
        Ok(files) => {
            let pb = ProgressBar::new(files.len().try_into().unwrap());
            for full_path in files {
                let status = Command::new("objcopy")
                    .arg("-O").arg("binary")
                    .arg("--only-section=.text")
                    .arg(&full_path)
                    .arg("/tmp/text.bin")
                    .status()
                    .unwrap_or_else(|err| panic!("Error running command: {}", err));

                if status.code().unwrap() != 0 { continue; }

                let mut file = File::open("/tmp/text.bin").unwrap();
                let mut buffer: Vec<u8> = Vec::new();
                file.read_to_end(&mut buffer).unwrap();
                decode_codes(&buffer, &mut counted_codes);
                pb.inc(1);
            }
            pb.finish_with_message("done");
        },
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
    println!("{:?}", counted_codes);
}
