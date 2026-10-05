//! Auxiliary formatting wrappers for types that ufmt doesn’t support directly.

use core::ffi::CStr;

use ufmt::{uDebug, uDisplay};

pub(crate) struct CStrFormatter<'a>(pub(crate) &'a CStr);

impl<'a> uDebug for CStrFormatter<'a> {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        for byte in self.0.to_bytes() {
            for chr in byte.escape_ascii() {
                let chr = chr as char;
                f.write_char(chr)?;
            }
        }
        Ok(())
    }
}

impl<'a> uDisplay for CStrFormatter<'a> {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        let str = self.0.to_string_lossy();
        f.write_str(&str)
    }
}

// From https://github.com/japaric/ufmt/pull/65
// Copyright (c) 2019 Jorge Aparicio & 2025 Markus Reiter
pub(crate) struct StrFormatter<'a>(pub(crate) &'a str);
impl<'a> uDebug for StrFormatter<'a> {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        f.write_str("\"")?;
        let mut from = 0;
        for (i, c) in self.0.char_indices() {
            let esc = c.escape_debug();

            // If char needs escaping, flush backlog so far and write, else skip
            if esc.len() != 1 {
                // SAFETY: `char_indices()` guarantees that `i` is always the index of utf-8 boundary of `c`.
                // In the first iteration `from` is zero and therefore also the index of the boundary of `c`.
                // In the following iterations `from` either keeps its value or is set to `i + c.len_utf8()`
                // (with last rounds `i` and `c`), which means `from` is again `i` (this round), the index
                // of this rounds `c`. Notice that this also implies `from <= i`.
                f.write_str(unsafe { self.0.get_unchecked(from..i) })?;
                for c in esc {
                    f.write_char(c)?;
                }
                from = i + c.len_utf8();
            }
        }

        // SAFETY: As seen above, `from` is the index of an utf-8 boundary in `self`.
        // This also means that from is in bounds of `self`: `from <= self.len()`.
        f.write_str(unsafe { self.0.get_unchecked(from..) })?;
        f.write_str("\"")
    }
}
