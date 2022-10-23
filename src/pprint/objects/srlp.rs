use crate::{pprint::{PPrintable, Printer}, SRLP, objects::Obj};

impl PPrintable for SRLP {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.field("Plot type").value(self.plot_type);
            p.field("Plant type").value(self.plant_type);
            p.field("Name").value(&self.name);
            p.field("Plot inventories").value(&self.plot_inventories);
            p.field("Upgrades").value(&self.upgrades);
            p.field("Tracked actor list")
                .value(&self.tracked_actor_list);

            p.ufield("Ran feeder").value(&self.ran_feeder);
            p.ufield("Ran vaccuum").value(&self.ran_vaccuum);

            p.ufield("Numbers").value(&self.numbers);
            p.ufield("Unknown").value(&self.unknown);
            p.ufield("Unknown2").value(&self.unknown2);
            p.ufield("f").value(self.f);
            p.ufield("b1").value(self.b1);
        })
    }
}
