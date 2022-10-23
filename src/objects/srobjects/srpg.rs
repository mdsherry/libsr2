use super::{simple_obj, Obj, Srv3};
use crate::objects::{ItemId, PPrintable, Parseable, Printer, SceneGroupId};
use nom::IResult;
use std::{fmt::Debug, io::Write};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SRPg {
    pub item_id: ItemId,
    pub actor_id: i32,
    pub c: i32,
    pub angle: f32,
    // Consider: could contain inventory if a warp depot, or pointer to other endpoint
    pub bytes: [u8; 32],
    pub scene_group_id: SceneGroupId,
    pub e: [u8; 5],
    pub f: Srv3,
}

simple_obj!(
    SRPg,
    "SRPG",
    item_id,
    actor_id,
    c,
    angle,
    bytes,
    scene_group_id,
    e,
    f
);

impl PPrintable for SRPg {
    fn pprint(&self, printer: &mut Printer) {
        printer.object("SRPG", |p| {
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
