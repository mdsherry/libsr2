use crate::{
    objects::Obj,
    pprint::{PPrintable, Printer},
    SRPL,
};

impl PPrintable for SRPL {
    fn pprint(&self, printer: &mut Printer) -> std::io::Result<()> {
        printer.object(Self::NAME, |p| {
            p.field("HP")?.value(self.hp)?;
            p.field("Energy")?.value(self.energy)?;
            p.field("Money")?.value(self.money)?;
            p.field("Build")?.value(&self.build)?;
            p.field("Pos")?.value(&self.pos)?;
            p.field("Facing")?.value(&self.facing)?;
            p.field("Total money earned")?
                .value(self.total_money_earned)?;
            p.field("Purchased upgrades")?.value(&self.srpu)?;

            p.field("Gadgets unlocked")?.value(&self.gadgets_unlocked)?;

            p.field("Refinery contents")?
                .value(&self.refinery_contents)?;
            p.field("Upgrade components")?
                .value(&self.upgrade_components)?;
            p.field("Inventory")?.value(&self.inventory)?;

            p.ufield("srdzr")?.value(&self.srdzr)?;
            p.ufield("a")?.value(self.a)?;
            p.ufield("c")?.value(self.c)?;
            p.ufield("b1")?.value(self.b1)?;
            p.ufield("b2")?.value(self.b2)?;
            p.ufield("unknown")?.value(&self.unknown)?;
            p.ufield("unknown")?.value(&self.unknown2)
        })
    }
}
