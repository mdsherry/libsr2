use crate::{
    pprint::{PPrintable, Printer},
    TRACKEDACTORLIST, SRGAME,
};

impl PPrintable<SRGAME> for TRACKEDACTORLIST {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> std::io::Result<()> {
        printer.object("TRACKEDACTORLIST", |p| {
            p.ufield("Actor IDs")?.list(&self.actor_ids, |p, id| {
                let id = *id as i32; // WTF
                if let Some((index, actor_type)) = p
                    .game()
                    .srads
                    .iter()
                    .filter_map(|actor| {
                        actor.as_v2().and_then(|srad| {
                            if srad.index == id {
                                Some((srad.index, srad.actor_type))
                            } else {
                                None
                            }
                        })
                    })
                    .next()
                {
                    index.pprint(p)?;
                    p.print(" (")?;
                    actor_type.pprint(p)?;
                    p.print(")")
                } else {
                    p.print(&format!("<no such actor {id}?>"))
                }
            })
        })
    }
}
