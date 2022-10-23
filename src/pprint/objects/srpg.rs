use crate::{pprint::{PPrintable, Printer}, SRPG, objects::Obj};

impl PPrintable for SRPG {
    fn pprint(&self, printer: &mut Printer) {
        printer.object(Self::NAME, |p| {
            p.field("Item ID").value(self.item_id);
            p.field("Scene group ID").value(self.scene_group_id);

            p.ufield("Actor ID?").value(self.actor_id);
            p.ufield("c").value(self.c);
            p.ufield("Angle").value(self.angle);
            p.ufield("bytes").value(&self.bytes);
            p.ufield("e").value(&self.e);
            p.ufield("f").value(&self.f);
        });
    }
}
