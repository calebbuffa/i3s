//! Little-endian binary reading and writing primitives shared by the
//! attribute and geometry codecs.
//!
//! I3S binary resources are **densely packed** and explicitly little-endian.
//! Packing is by byte, not by natural alignment: a `color` block declared as
//! `UInt8 * 3` leaves the following `Float32` block starting on a byte offset
//! that is not a multiple of four. Every read here therefore goes through
//! `from_le_bytes` on a byte slice rather than a pointer cast, so unaligned
//! data is handled correctly instead of panicking.

/// Minimal zero-copy cursor over a byte slice.
pub(crate) struct BufferReader<'a> {
    data: &'a [u8],
    pos: usize,
}

/// The buffer ended before the requested bytes could be read.
#[derive(Debug)]
pub(crate) struct UnexpectedEndOfData;

impl<'a> BufferReader<'a> {
    pub(crate) fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    /// Bytes not yet consumed.
    pub(crate) fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }

    /// Moves the cursor to an absolute offset, e.g. to honour
    /// `geometryBuffer.offset` or a `byteOffset` on a legacy attribute.
    pub(crate) fn seek(&mut self, pos: usize) -> Result<(), UnexpectedEndOfData> {
        if pos > self.data.len() {
            return Err(UnexpectedEndOfData);
        }
        self.pos = pos;
        Ok(())
    }

    pub(crate) fn read_le<T: LeBytes>(&mut self) -> Result<T, UnexpectedEndOfData> {
        let end = self.pos.checked_add(T::SIZE).ok_or(UnexpectedEndOfData)?;
        if end > self.data.len() {
            return Err(UnexpectedEndOfData);
        }
        let val = T::from_le(&self.data[self.pos..end]);
        self.pos = end;
        Ok(val)
    }

    pub(crate) fn read_bytes(&mut self, n: usize) -> Result<&'a [u8], UnexpectedEndOfData> {
        let end = self.pos.checked_add(n).ok_or(UnexpectedEndOfData)?;
        if end > self.data.len() {
            return Err(UnexpectedEndOfData);
        }
        let slice = &self.data[self.pos..end];
        self.pos = end;
        Ok(slice)
    }

    pub(crate) fn read_le_vec<T: LeBytes>(
        &mut self,
        count: usize,
    ) -> Result<Vec<T>, UnexpectedEndOfData> {
        // Reserve exactly once; `count` is derived from a header value, so it
        // is bounded by a prior length check to avoid a hostile allocation.
        let end = self
            .pos
            .checked_add(count.checked_mul(T::SIZE).ok_or(UnexpectedEndOfData)?)
            .ok_or(UnexpectedEndOfData)?;
        if end > self.data.len() {
            return Err(UnexpectedEndOfData);
        }
        let mut out = Vec::with_capacity(count);
        for _ in 0..count {
            out.push(self.read_le::<T>()?);
        }
        Ok(out)
    }
}

/// A fixed-width value with a little-endian byte representation.
pub(crate) trait LeBytes: Sized + Copy {
    const SIZE: usize;
    fn from_le(bytes: &[u8]) -> Self;
    fn write_le(self, out: &mut Vec<u8>);
}

macro_rules! impl_le_bytes {
    ($($t:ty),*) => {
        $(impl LeBytes for $t {
            const SIZE: usize = std::mem::size_of::<$t>();
            fn from_le(bytes: &[u8]) -> Self {
                Self::from_le_bytes(bytes.try_into().unwrap())
            }
            fn write_le(self, out: &mut Vec<u8>) {
                out.extend_from_slice(&self.to_le_bytes());
            }
        })*
    };
}
impl_le_bytes!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

/// Appends `values` to `out` in little-endian order.
pub(crate) fn write_le_slice<T: LeBytes>(out: &mut Vec<u8>, values: &[T]) {
    out.reserve(values.len() * T::SIZE);
    for value in values {
        value.write_le(out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_unaligned_f32_after_three_byte_block() {
        // A 3-component UInt8 colour leaves the cursor at offset 3, which is
        // not 4-byte aligned. A pointer cast would be UB here; this must work.
        let mut bytes = vec![1u8, 2, 3];
        bytes.extend_from_slice(&1.5f32.to_le_bytes());
        let mut reader = BufferReader::new(&bytes);
        assert_eq!(reader.read_le_vec::<u8>(3).unwrap(), vec![1, 2, 3]);
        assert_eq!(reader.remaining(), 4);
        assert_eq!(reader.read_le::<f32>().unwrap(), 1.5);
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn oversized_count_is_rejected_without_allocating() {
        let bytes = [0u8; 8];
        let mut reader = BufferReader::new(&bytes);
        assert!(reader.read_le_vec::<f32>(usize::MAX).is_err());
        assert!(reader.read_le_vec::<f32>(3).is_err());
        // A rejected read must not advance the cursor.
        assert_eq!(reader.remaining(), 8);
    }

    #[test]
    fn seek_past_end_is_rejected() {
        let bytes = [0u8; 4];
        let mut reader = BufferReader::new(&bytes);
        assert!(reader.seek(4).is_ok());
        assert!(reader.seek(5).is_err());
    }

    #[test]
    fn write_le_slice_round_trips() {
        let mut out = Vec::new();
        write_le_slice(&mut out, &[1.0f32, -2.5, 3.25]);
        let mut reader = BufferReader::new(&out);
        assert_eq!(reader.read_le_vec::<f32>(3).unwrap(), vec![1.0, -2.5, 3.25]);
    }
}
