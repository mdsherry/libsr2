use std::{
    fs::File,
    io::{self, BufWriter},
    path::Path,
};

use crate::SRGame;

use super::PPrintable;

pub struct Printer<'a> {
    margin: usize,
    game: &'a SRGame,
    out: Box<dyn io::Write>,
    new_line: bool,
    compare_mode: bool,
}

impl<'a> Printer<'a> {
    pub fn with_filename(mut self, outfile: &Path) -> io::Result<Self> {
        self.out = Box::new(BufWriter::new(File::create(outfile)?));
        Ok(self)
    }
    pub fn set_compare_mode(&mut self, enabled: bool) {
        self.compare_mode = enabled;
    }
    pub fn compare_mode(&self) -> bool {
        self.compare_mode
    }
    pub fn new(game: &'a SRGame) -> Self {
        Printer {
            margin: 0,
            game,
            new_line: true,
            out: Box::new(BufWriter::new(io::stdout().lock())),
            compare_mode: false,
        }
    }
    pub fn game(&self) -> &SRGame {
        self.game
    }
    fn ensure_nl(&mut self) -> io::Result<()> {
        if self.new_line {
            write!(self.out, "{0:1$}", ' ', self.margin)?;
            self.new_line = false;
        }
        Ok(())
    }
    pub fn print(&mut self, text: &str) -> io::Result<()> {
        self.ensure_nl()?;
        self.out.write_all(text.as_bytes())?;
        Ok(())
    }

    pub fn nl(&mut self) -> io::Result<()> {
        if !self.new_line {
            writeln!(self.out)?;
            self.new_line = true;
        }
        Ok(())
    }
    pub fn list<P, F: FnMut(&mut Self, &P) -> io::Result<()>>(
        &mut self,
        items: &[P],
        mut f: F,
    ) -> io::Result<()> {
        if items.is_empty() {
            self.print("<empty>")?;
        } else {
            self.margin += 2;
            let width = width(items.len());
            for (i, item) in items.iter().enumerate() {
                self.nl()?;
                self.ensure_nl()?;
                if self.compare_mode {
                    write!(self.out, "- ")?;
                } else {
                    write!(self.out, "[{i:>0$}] ", width as usize)?;
                }
                f(self, item)?;
            }
            self.margin -= 2;
        }
        self.nl()?;
        Ok(())
    }
    pub fn map<
        'data,
        K: 'data,
        V: 'data,
        F: Fn(&mut Self, &'data (K, V)) -> io::Result<()>,
        I: IntoIterator<Item = &'data (K, V)>,
    >(
        &mut self,
        items: I,
        f: F,
    ) -> io::Result<()> {
        self.margin += 2;
        let mut any = false;
        for item in items {
            any = true;
            self.nl()?;
            self.print("- ")?;
            f(self, item)?;
        }
        if !any {
            self.print("<empty>")?;
        }
        self.margin -= 2;
        self.nl()?;
        Ok(())
    }

    pub fn object<F>(&mut self, object: &str, f: F) -> io::Result<()>
    where
        F: FnOnce(&mut Self) -> io::Result<()>,
    {
        self.ensure_nl()?;
        self.print(&format!("{object} {{"))?;
        self.nl()?;
        self.margin += 2;
        f(self)?;
        self.margin -= 2;
        self.nl()?;
        self.print("}")?;
        Ok(())
    }

    pub fn field(&mut self, name: &str) -> io::Result<&mut Self> {
        self.nl()?;
        self.ensure_nl()?;
        write!(self.out, "{name}: ")?;
        Ok(self)
    }
    pub fn ufield(&mut self, name: &str) -> io::Result<&mut Self> {
        self.nl()?;
        self.ensure_nl()?;
        write!(self.out, "?{name}?: ")?;
        Ok(self)
    }
    pub fn value<D: PPrintable>(&mut self, value: D) -> io::Result<()> {
        self.ensure_nl()?;
        value.pprint(self)?;
        Ok(())
    }
}

fn width(n: usize) -> u32 {
    if n >= 10000000 {
        8
    } else if n >= 1000000 {
        7
    } else if n >= 100000 {
        6
    } else if n >= 10000 {
        5
    } else if n >= 1000 {
        4
    } else if n >= 100 {
        3
    } else if n >= 10 {
        2
    } else {
        1
    }
}
