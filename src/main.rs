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
use libsr2::{PPrintable, Parseable, Printer, SRGame};

#[derive(Debug, Clone, Parser)]
struct Args {
    filename: PathBuf,
    #[clap(long, short)]
    comparison: bool,
}

fn load_data(path: &Path) -> color_eyre::eyre::Result<SRGame> {
    let data = std::fs::read(path)?;

    let (_, game) = SRGame::parse(&data)
        .map_err(|e| match e {
            nom::Err::Incomplete(_) => todo!(),
            nom::Err::Error(e) | nom::Err::Failure(e) => {
                eprintln!("Error code: {:?}", e.code);
                eprintln!("Error location: {:x}", data.len() - e.input.len());
                let s: String = e
                    .input
                    .iter()
                    .take(30)
                    .map(|b| {
                        if b.is_ascii_graphic() || *b == 0x20 {
                            *b as char
                        } else {
                            '★'
                        }
                    })
                    .collect();
                eprintln!("Output: {s}");
            }
        })
        .unwrap();
    Ok(game)
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
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
    // let mut h: HashMap<u16, usize> = HashMap::new();
    // for srad in &game.srads {
    //     match srad {
    //         objects::SRad::V1 { .. } => (),
    //         objects::SRad::V2 { index, actor_type, more_bytes, even_more_bytes, .. } => {
    //             if *more_bytes != [0; 5] || *even_more_bytes != [0; 4] {
    //                 printer.field("Index")?.value(index);
    //                 printer.field("Actor type")?.value(actor_type);
    //                 printer.field("More bytes")?.value(more_bytes);
    //                 printer.field("Even more bytes")?.value(even_more_bytes);
    //                 printer.nl();
    //             }
    //         },
    //     }
    // }
    // dbg!(h);
    // let mut window = Window::new("Kiss3d: cube");
    // let mut scene = window.add_group();
    // for (pos, _time) in &game.srw.hen_spawner_timers.0 {
    //     let mut c = scene.add_sphere(1.);
    //     c.set_local_translation([pos.coords[0], pos.coords[1], -pos.coords[2]].into());
    //     c.set_color(1., 1., 0.);
    // }
    // for (pos, _time) in &game.srw.slime_spawner_timers.0 {
    //     let mut c = scene.add_sphere(1.);
    //     c.set_local_translation([pos.coords[0], pos.coords[1], -pos.coords[2]].into());
    //     c.set_color(1., 1., 1.);
    // }
    // for srad in &game.srads {
    //     if let Some(srad) = srad.as_v2() {
    //         if srad.zone.0 != 1 { continue; }
    //         let mut c = scene.add_sphere(1.);
    //         c.set_local_translation([srad.pos.coords[0], srad.pos.coords[1], -srad.pos.coords[2]].into());
    //         let actor_type_name = &game.item_index.identifiable_types[srad.actor_type.0];
    //         if actor_type_name.starts_with("SlimeDefinition") {
    //             c.set_color(0., 1., 0.);
    //         } else if actor_type_name.ends_with("Plort") {
    //             c.set_color(0., 1., 1.);
    //         } else if actor_type_name.starts_with("IdentifiableType.Container") {
    //             c.set_color(0., 0., 1.);
    //         } else if actor_type_name.ends_with("Hen") || actor_type_name.ends_with("Rooster") || actor_type_name.ends_with("Chick") {
    //             c.set_color(1., 0., 1.);
    //         } else {
    //             c.set_color(1., 0., 0.);
    //         }
    //     }

    // }
    // let mut tracked = vec![];
    // for plot in &game.ranch.plots {
    //     tracked.extend(plot.tracked_actor_list.actor_ids.iter().map(|id| *id as i32));
    // }
    // game.srads.retain(|srad| {
    //     srad.as_v2().map(|s| tracked.contains(&s.index)).unwrap_or(false)
    // });
    // let mut f = BufWriter::new(File::create(args.filename.file_name().unwrap())?);
    // game.write(&mut f);
    // window.set_light(Light::StickToCamera);

    // let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    // let state = AppState { c: scene, rot };

    // window.render_loop(state);

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
