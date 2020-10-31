pub use error::{Error, Result};

pub mod elements;

mod error;

use std::io;

struct ToWatParams {
    indent_size: usize,
    indent_level: usize,
}

impl ToWatParams {
    fn indent(&self) -> usize {
        self.indent_size * self.indent_level
    }
}

impl Default for ToWatParams {
    fn default() -> Self {
        Self {
            indent_size: 2,
            indent_level: 0,
        }
    }
}

trait ToWat {
    fn write_wat<W: io::Write>(
        &self,
        w: &mut W,
        p: &ToWatParams,
    ) -> io::Result<()>;

    fn to_wat(&self, p: &ToWatParams) -> String {
        let mut buf = Vec::new();

        self.write_wat(&mut buf, p).unwrap();

        let s = String::from_utf8_lossy(&buf);

        s.to_string()
    }
}
