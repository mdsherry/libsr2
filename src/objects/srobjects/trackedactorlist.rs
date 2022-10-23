use super::{simple_obj, Obj, SRAD};
use crate::objects::{PPrintable, Parseable, Printer};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TrackedActorList {
    pub actor_ids: Vec<f64>,
}
simple_obj!(TrackedActorList, "TRACKEDACTORLIST", actor_ids);
impl PPrintable for TrackedActorList {
    fn pprint(&self, printer: &mut Printer) {
        printer.object("TRACKEDACTORLIST", |p| {
            p.ufield("Actor IDs").list(&self.actor_ids, |p, id| {
                let id = *id as i32; // WTF
                if let Some((index, actor_type)) = p
                    .game()
                    .srads
                    .iter()
                    .filter_map(|actor| 
                        actor.as_v2().and_then(|srad| 
                            if srad.index == id {
                                Some((srad.index, srad.actor_type))
                            } else {
                                None
                            }
                        )
                    )
                    .next()
                {
                    index.pprint(p);
                    p.print(" (");
                    actor_type.pprint(p);
                    p.print(")");
                } else {
                    p.print(&format!("<no such actor {id}?>"));
                }
            });
        });
    }
}
