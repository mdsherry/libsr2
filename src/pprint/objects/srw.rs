use crate::{pprint::{PPrintable, Printer}, Srw, objects::Obj};

impl PPrintable for Srw {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("World time")?.value(self.world_time)?;
            p.field("Plort market saturation")?
                .value(&self.plort_market_saturation)?;
            p.field("Liquid sources")?.value(&self.liquid_sources)?;
            p.field("Gordos")?.value(&self.gordos)?;
            p.field("Gadgets")?.value(&self.gadgets)?;
            p.field("Pods")?.value(&self.pods)?;
            p.field("Switches")?.value(&self.switches)?;
            p.field("Puzzles")?.value(&self.puzzles)?;
            p.field("Research drones")?.value(&self.research_drones)?;
            p.field("Resource nodes")?.value(&self.resource_nodes)?;

            
            p.ufield("a")?.value(&self.a)?;
            p.ufield("b")?.value(&self.b)?;
            p.ufield("c")?.value(&self.c)?;
            p.ufield("d")?.value(&self.d)?;
            p.ufield("e")?.value(&self.e)?;
            

            p.ufield("Hen spawner timers")?.value(&self.hen_spawner_timers)?;
            p.ufield("Slime spawner timers")?.value(&self.slime_spawner_timers)?;
            p.ufield("map5")?.value(&self.map5)?;
            p.ufield("unknown2")?.value(&self.unknown2)?;
            p.ufield("unknown3")?.value(&self.unknown3)
        })
    }
}
