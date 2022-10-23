use crate::{pprint::{PPrintable, Printer}, SRAD, SRAD1, SRAD2};

impl PPrintable for SRAD {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        match self {
            SRAD::V1(v1) => v1.pprint(printer),
            SRAD::V2(v2) => v2.pprint(printer),
        }
    }
}

impl PPrintable for SRAD1 {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object("SRAD::V1", |p| {
            p.field("Item ID")?.value(self.item_id)?;
            p.field("Count")?.value(self.count)?;
            p.ufield("c")?.value(self.c)?;
            p.ufield("sed")?.value(&self.sed)
        })
    }
}

impl PPrintable for SRAD2 {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object("SRAD::V2", |p| {
            p.field("Pos")?.value(&self.pos)?;
            p.field("Facing")?.value(&self.facing)?;
            p.field("Actor type")?.value(self.actor_type)?;
            p.ufield("Index")?.value(self.index)?;
            p.ufield("unknown")?.value(self.unknown)?;
            p.ufield("srsed")?.value(&self.srsed)?;
            p.ufield("Chick timer")?.value(self.chick_timer)?;
            p.ufield("Hen timer")?.value(self.hen_timer)?;
            p.ufield("Plort expires")?.value(self.plort_expire)?;
            p.ufield("Plant state")?.value(&self.srrcd)?;
            p.field("Feral")?.value(self.feral)?;

            if let Some(ts) = self.ts4 {
                p.ufield("ts4")?.value(ts)?;
            }
            p.ufield("more_bytes")?.value(self.more_bytes.as_slice())?;
            p.ufield("zone")?.value(self.zone)?;
            p.field("Statuified")?.value(self.statuified)?;
            p.field("Statued actor ID")?.value(self.statue_actor_id)?;
            p.ufield("even_more_bytes")
                ?.value(self.even_more_bytes.as_slice())?;
            p.ufield("ts5")?.value(self.ts5)?;
            p.ufield("srses")?.value(&self.srses)
        })
    }
}
