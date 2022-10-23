use chrono::{prelude::*, Duration};

use crate::primitives::{VecMap, ItemId, InGameTime, TimeSinceYear1, PlantId, SceneGroupId, WindowsTime};

use super::{PPrintable, Printer};

impl PPrintable for i32 {
    fn pprint(&self, printer: &mut Printer) {
        printer.print(&format!("{self}"));
    }
}

impl PPrintable for f32 {
    fn pprint(&self, printer: &mut Printer) {
        printer.print(&format!("{self}"));
    }
}
impl PPrintable for f64 {
    fn pprint(&self, printer: &mut Printer) {
        printer.print(&format!("{self}"));
    }
}

impl PPrintable for u64 {
    fn pprint(&self, printer: &mut Printer) {
        printer.print(&format!("{self}"));
    }
}
impl PPrintable for String {
    fn pprint(&self, printer: &mut Printer) {
        printer.print(&format!("{self}"));
    }
}
impl PPrintable for bool {
    fn pprint(&self, printer: &mut Printer) {
        printer.print(&format!("{self}"));
    }
}
impl PPrintable for &[u8] {
    fn pprint(&self, printer: &mut Printer) {
        printer.print(&format!("{self:02x?}"));
    }
}
impl<const N: usize> PPrintable for [u8; N] {
    fn pprint(&self, printer: &mut Printer) {
        printer.print(&format!("{self:02x?}"));
    }
}


impl<K: PPrintable, V: PPrintable> PPrintable for VecMap<K, V> {
    fn pprint(&self, printer: &mut Printer) {
        printer.map(&self.0, |p, (k, v)| {
            k.pprint(p);
            p.print(": ");
            v.pprint(p);
        })
    }
}
impl PPrintable for ItemId {
    fn pprint(&self, printer: &mut Printer) {
        let s = if self.0 != usize::MAX {
            printer.game().item_index.identifiable_types[self.0].clone()
        } else {
            "<empty>".to_string()
        };
        printer.print(&s);
    }
}


impl PPrintable for InGameTime {
    fn pprint(&self, printer: &mut Printer) {
        const SECONDS_PER_MINUTE: f64 = 60.;
        const SECONDS_PER_HOUR: f64 = 60. * SECONDS_PER_MINUTE;
        const SECONDS_PER_DAY: f64 = 24. * SECONDS_PER_HOUR;
        let d = (self.0 / SECONDS_PER_DAY) as i32;
        let h = ((self.0 % SECONDS_PER_DAY) / SECONDS_PER_HOUR) as i32;
        let m = ((self.0 % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE) as i32;
        let s = self.0 % SECONDS_PER_MINUTE;
        if self.0.is_nan() {
            printer.print("NaN");
        } else {
            printer.print(&format!("{d}d {h:02}:{m:02}:{s:02.2}"));
        }
    }
}


impl PPrintable for TimeSinceYear1 {
    fn pprint(&self, printer: &mut Printer) {
        let epoch = chrono::Utc.ymd(1, 1, 1).and_hms(0, 0, 0);
        let ts = epoch + Duration::seconds(self.0 / 10_000_000);
        printer.print(&format!("{}", ts.format("%Y-%m-%dT%H:%M:%S%z")));
    }
}

impl PPrintable for WindowsTime {
    fn pprint(&self, printer: &mut Printer) {
        let epoch = chrono::Utc.ymd(1601, 1, 1).and_hms(0, 0, 0);
        let ts = epoch + Duration::seconds(self.0 / 10_000_000);
        printer.print(&format!("{}", ts.format("%Y-%m-%dT%H:%M:%S%z")));
    }
}

impl PPrintable for SceneGroupId {
    fn pprint(&self, printer: &mut Printer) {
        let s = if self.0 != usize::MAX {
            printer.game().scene_group_index.scene_groups[self.0].clone()
        } else {
            "<no zone>".to_string()
        };
        printer.print(&s);
    }
}

impl PPrintable for PlantId {
    fn pprint(&self, printer: &mut Printer) {
        let s = if self.0 != usize::MAX {
            printer.game().patch_index.patches[self.0].clone()
        } else {
            "<no plant>".into()
        };
        printer.print(&s);
    }
}