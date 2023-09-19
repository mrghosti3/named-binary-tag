/// Module with helper functions for compressing NBT format (deflating).
pub mod deflate {
    use crate::encode::write_compound_tag;
    use crate::CompoundTag;
    use flate2::write::{GzEncoder, ZlibEncoder};
    use std::io::{Error, Write};

    /// Write a compound tag to writer using gzip compression.
    pub fn write_gzip_compound_tag<W: Write>(
        writer: &mut W,
        compound_tag: &CompoundTag,
    ) -> Result<(), Error> {
        let mut encoder = GzEncoder::new(writer, Default::default());
        let result = write_compound_tag(&mut encoder, compound_tag);
        encoder.finish()?;
        result
    }

    /// Write a compound tag to writer using zlib compression.
    pub fn write_zlib_compound_tag<W: Write>(
        writer: &mut W,
        compound_tag: &CompoundTag,
    ) -> Result<(), Error> {
        write_compound_tag(
            &mut ZlibEncoder::new(writer, Default::default()),
            compound_tag,
        )
    }
}

/// Module with helper functions for decompressing NBT format (enflating).
pub mod enflate {
    use crate::decode::{read_compound_tag, TagDecodeError};
    use crate::CompoundTag;
    use flate2::read::{GzDecoder, ZlibDecoder};
    use std::io::Read;

    /// Read a compound tag from a reader compressed with gzip.
    pub fn read_gzip_compound_tag<R: Read>(reader: &mut R) -> Result<CompoundTag, TagDecodeError> {
        read_compound_tag(&mut GzDecoder::new(reader))
    }

    /// Read a compound tag from a reader compressed with zlib.
    pub fn read_zlib_compound_tag<R: Read>(reader: &mut R) -> Result<CompoundTag, TagDecodeError> {
        read_compound_tag(&mut ZlibDecoder::new(reader))
    }
}
