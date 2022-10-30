use std::{fmt::Write, array};
#[allow(unused_imports)]
use std::{
    collections::HashMap,
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};

use clap::Parser;
#[allow(unused_imports)]
use kiss3d::{
    light::Light,
    nalgebra::{UnitQuaternion, Vector3},
    scene::SceneNode,
    window::{State, Window},
};
use libsr2::{PPrintable, Parseable, Printer, SRGAME, v1::{SRGAME1 as SRGAME1, self}};
use nom::{Finish, error::VerboseError};

#[derive(Debug, Clone, Parser)]
struct Args {
    filename: PathBuf,
    #[clap(long, short)]
    comparison: bool,
}

fn decorate(bytes: &[u8], length: usize) -> String {
    let mut s: String = String::new();
    s.push_str("       ");
    for (i, _) in bytes.iter().take(length).enumerate() {
        if i % 8 == 0{
            s.push_str(" : ");
        } else if i % 4 == 0 {
            s.push_str(" . ");
        } else {
            s.push_str("   ");
        }
    }
    s.push_str("\nInput: ");
    bytes
        .iter()
        .take(length)
        .for_each(|b| {
            s.push(' ');
            if b.is_ascii_graphic() || *b == 0x20 {
                s.push(*b as char)
            } else {
                s.push('.')
            };
            s.push(' ');
        });
    s.push_str("\n       ");
    bytes
        .iter()
        .take(length)
        .for_each(|b| {
            write!(&mut s, "{:02x} ", b);
        });
    s.push_str("\n       ");
    let mut last_comma = None;
    let mut just_wrote_tag = false;
    bytes
        .windows(2).enumerate()
        .take(length)
        .for_each(|(i, b)| {
            if b == &[0x00, 0x30] {
                write!(&mut s, ">>>>> ");
                just_wrote_tag = true;
            } else if b == &[0x00, 0x40] {
                write!(&mut s, "<<<<< ");
                just_wrote_tag = true;
            } else if b == &[0x00, 0x20] {
                // write!(&mut s3, "||||| ");
                if let Some(last_comma) = last_comma {
                    // println!("Distance between commas: {}", i - last_comma - 2);
                    
                    write!(&mut s, "|{}| ", format!("{:3}", i - last_comma - 2).replace(' ', "|"));
                } else{
                    write!(&mut s, "||||| ");
                }
                last_comma = Some(i);
                just_wrote_tag = true;
            } else if !just_wrote_tag {
                write!(&mut s, "   ");
            } else {
                just_wrote_tag = false;
            }
            
        });
    
    s
}

fn err_dump(data: &[u8], e: VerboseError<&[u8]>) {
    for (input, err) in e.errors.iter().rev() {
                
        eprintln!("Error code: {:?}", err);
        eprintln!("Error location: {0:05x} ({0})", data.len() - input.len());
    }
    if let Some((input, err)) = e.errors.first() {
        eprintln!("Previously:");
        eprintln!("{}", decorate(&data[(data.len() - input.len()).saturating_sub(100)..data.len() - input.len()], 100));
        eprintln!("{}", decorate(input, 50));
        if input.len() >= 4 {
            println!("Next 4 bytes as f32: {}", f32::from_le_bytes(array::from_fn(|i| input[i])));
            println!("Next 4 bytes as i32: {}", i32::from_le_bytes(array::from_fn(|i| input[i])));
        }
        if input.len() >= 8 {
            println!("Next 8 bytes as f64: {}", f64::from_le_bytes(array::from_fn(|i| input[i])));
            println!("Next 8 bytes as i64: {}", i64::from_le_bytes(array::from_fn(|i| input[i])));
        }
        
    }
}

fn load_data(path: &Path) -> color_eyre::eyre::Result<SRGAME> {
    let data = std::fs::read(path)?;
    let v2game = Default::default();
    let mut printer = Printer::new(&v2game).with_filename("testout.txt".as_ref())?;
    
    
    // for i in 0..data.len() {
    //     if data[i..].starts_with(&[0x04, 0x53, 0x52, 0x50, 0x47]) {
    //         println!("{}", decorate(&data[i + 11..], 200));
    //         match v1::SRPG::parse(&data[i..]).finish() {
    //             Ok((_, srpg)) => srpg.pprint(&mut printer)?,
    //             Err(e) => err_dump(&data[i..], e),
    //         }

    //     }
    // }
    let (_, game) = SRGAME1::parse(&data).finish()
        .map_err(|e| err_dump(&data, e))
        .unwrap();
        
    // let mut printer = Printer::new(&v2game).with_filename("testout.txt".as_ref())?;
    
    game.pprint(&mut printer)?;
    // Ok(game)
    unimplemented!()
}

fn main() -> color_eyre::eyre::Result<()> {
    // color_eyre::install()?;
    let args = Args::parse();
    let game = load_data(&args.filename)?;

    let outfile_name_base: Option<&Path> = args.filename.file_stem().map(|p| p.as_ref());
    let outfile_name = outfile_name_base
        .unwrap_or("save".as_ref())
        .with_extension("txt");
    let mut printer = Printer::new(&game).with_filename(&outfile_name)?;
    printer.set_compare_mode(args.comparison);
    game.pprint(&mut printer)?;
    // game.gsumm.pprint(&mut printer);
    printer.nl()?;
   
    Ok(())
}

// struct AppState {
//     c: SceneNode,
//     rot: UnitQuaternion<f32>,
// }

// impl State for AppState {
//     fn step(&mut self, _: &mut Window) {
//         // self.c.prepend_to_local_rotation(&self.rot)
//     }
// }
