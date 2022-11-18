use std::io;

use chrono::{prelude::*, Duration};

use crate::{primitives::{
    InGameTime, ItemId, PlantId, SceneGroupId, TimeSinceYear1, VecMap, WindowsTime, UpgradeComponentId,
}, SRGAME};

use super::{PPrintable, Printer};

impl<T> PPrintable<T> for i32 {
    fn pprint(&self, printer: &mut Printer<T>) -> io::Result<()> {
        printer.print(&format!("{self}"))
    }
}

impl<T> PPrintable<T> for f32 {
    fn pprint(&self, printer: &mut Printer<T>) -> io::Result<()> {
        printer.print(&format!("{self}"))
    }
}
impl<T> PPrintable<T> for f64 {
    fn pprint(&self, printer: &mut Printer<T>) -> io::Result<()> {
        printer.print(&format!("{self}"))
    }
}

impl<T> PPrintable<T> for u64 {
    fn pprint(&self, printer: &mut Printer<T>) -> io::Result<()> {
        printer.print(&format!("{self}"))
    }
}
impl<T> PPrintable<T> for String {
    fn pprint(&self, printer: &mut Printer<T>) -> io::Result<()> {
        printer.print(&self.to_string())
    }
}
impl<T> PPrintable<T> for bool {
    fn pprint(&self, printer: &mut Printer<T>) -> io::Result<()> {
        printer.print(&format!("{self}"))
    }
}
impl<T> PPrintable<T> for &[u8] {
    fn pprint(&self, printer: &mut Printer<T>) -> io::Result<()> {
        printer.print(&format!("{self:02x?}"))
    }
}
impl<T, const N: usize> PPrintable<T> for [u8; N] {
    fn pprint(&self, printer: &mut Printer<T>) -> io::Result<()> {
        printer.print(&format!("{self:02x?}"))
    }
}

impl<T, K: PPrintable<T>, V: PPrintable<T>> PPrintable<T> for VecMap<K, V> {
    fn pprint(&self, printer: &mut Printer<T>) -> io::Result<()> {
        printer.map(&self.0, |p, (k, v)| {
            k.pprint(p)?;
            p.print(": ")?;
            v.pprint(p)
        })
    }
}
impl PPrintable<SRGAME> for ItemId {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> io::Result<()> {
        let s = if self.0 != usize::MAX {
            printer.game().item_index.identifiable_types[self.0].clone()
        } else {
            "<empty>".to_string()
        };
        printer.print(&s)
    }
}
impl PPrintable<SRGAME> for UpgradeComponentId {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> io::Result<()> {
        let s = if self.0 != usize::MAX {
            printer.game().uc_index.upgrade_components[self.0].clone()
        } else {
            "<empty>".to_string()
        };
        printer.print(&s)
    }
}


impl<T> PPrintable<T> for InGameTime {
    fn pprint(&self, printer: &mut Printer<T>) -> io::Result<()> {
        const SECONDS_PER_MINUTE: f64 = 60.;
        const SECONDS_PER_HOUR: f64 = 60. * SECONDS_PER_MINUTE;
        const SECONDS_PER_DAY: f64 = 24. * SECONDS_PER_HOUR;
        let d = (self.0 / SECONDS_PER_DAY) as i32;
        let h = ((self.0 % SECONDS_PER_DAY) / SECONDS_PER_HOUR) as i32;
        let m = ((self.0 % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE) as i32;
        let s = self.0 % SECONDS_PER_MINUTE;
        if self.0.is_nan() {
            printer.print("NaN")
        } else {
            printer.print(&format!("{d}d {h:02}:{m:02}:{s:02.2}"))
        }
    }
}

impl PPrintable<SRGAME> for TimeSinceYear1 {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> io::Result<()> {
        let epoch = chrono::Utc.ymd(1, 1, 1).and_hms(0, 0, 0);
        let ts = epoch + Duration::seconds(self.0 / 10_000_000);
        printer.print(&format!("{}", ts.format("%Y-%m-%dT%H:%M:%S%z")))
    }
}

impl PPrintable<SRGAME> for WindowsTime {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> io::Result<()> {
        let epoch = chrono::Utc.ymd(1601, 1, 1).and_hms(0, 0, 0);
        let ts = epoch + Duration::seconds(self.0 / 10_000_000);
        printer.print(&format!("{}", ts.format("%Y-%m-%dT%H:%M:%S%z")))
    }
}

impl PPrintable<SRGAME> for SceneGroupId {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> io::Result<()> {
        let s = if self.0 != usize::MAX {
            printer.game().scene_group_index.scene_groups[self.0].clone()
        } else {
            "<no zone>".to_string()
        };
        printer.print(&s)
    }
}

impl PPrintable<SRGAME> for PlantId {
    fn pprint(&self, printer: &mut Printer<SRGAME>) -> io::Result<()> {
        let s = if self.0 != usize::MAX {
            printer.game().patch_index.patches[self.0].clone()
        } else {
            "<no plant>".into()
        };
        printer.print(&s)?;
        Ok(())
    }
}
