use crate::{CompoundTag, Tag};
use byteorder::{BigEndian, ReadBytesExt};
use linked_hash_map::LinkedHashMap;
use std::{error::Error, io::Read};
use std::{fmt::Display, io};

/// Possible types of errors while decoding tag.
#[derive(Debug)]
pub enum TagDecodeError {
    /// Root of tag must be compound tag.
    RootMustBeCompoundTag {
        /// Actual tag.
        actual_tag: Tag,
    },
    /// Tag type not recognized.
    UnknownTagType {
        /// Tag type id which is not recognized.
        tag_type_id: u8,
    },
    /// I/O Error which happened while were decoding.
    IOError { io_error: io::Error },
}

impl From<io::Error> for TagDecodeError {
    fn from(io_error: io::Error) -> Self {
        TagDecodeError::IOError { io_error }
    }
}

impl Error for TagDecodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TagDecodeError::IOError { io_error } => Some(io_error),
            _ => None,
        }
    }
}

impl Display for TagDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RootMustBeCompoundTag { actual_tag } => write!(
                f,
                "Root must be a TAG_Compound but is a {}",
                actual_tag.type_name()
            ),
            Self::UnknownTagType { tag_type_id } => write!(f, "Unknown tag type: {}", tag_type_id),
            Self::IOError { .. } => write!(f, "IO Error"),
        }
    }
}

/// Read a compound tag from a reader.
///
/// # Example
/// ```
/// use nbt::decode::read_compound_tag;
/// use std::io::Cursor;
///
/// let mut cursor = Cursor::new(include_bytes!("../test/binary/servers.dat").to_vec());
/// let root_tag = read_compound_tag(&mut cursor).unwrap();
///
/// let servers = root_tag.get_compound_tag_vec("servers").unwrap();
/// assert_eq!(servers.len(), 1);
///
/// let server = servers[0];
/// let ip = server.get_str("ip").unwrap();
/// let name = server.get_str("name").unwrap();
/// let hide_address = server.get_bool("hideAddress").unwrap();
///
/// assert_eq!(ip, "localhost:25565");
/// assert_eq!(name, "Minecraft Server");
/// assert!(hide_address);
/// ```
pub fn read_compound_tag<R: Read>(reader: &mut R) -> Result<CompoundTag, TagDecodeError> {
    let tag_id = reader.read_u8()?;
    let name = read_string(reader)?;
    let tag = read_tag(tag_id, Some(name.as_str()), reader)?;

    match tag {
        Tag::Compound(value) => Ok(value),
        actual_tag => Err(TagDecodeError::RootMustBeCompoundTag { actual_tag }),
    }
}

fn read_tag<R: Read>(
    tag_id: u8,
    name: Option<&str>,
    reader: &mut R,
) -> Result<Tag, TagDecodeError> {
    match tag_id {
        1 => {
            let value = reader.read_i8()?;

            Ok(Tag::Byte(value))
        }
        2 => {
            let value = reader.read_i16::<BigEndian>()?;

            Ok(Tag::Short(value))
        }
        3 => {
            let value = reader.read_i32::<BigEndian>()?;

            Ok(Tag::Int(value))
        }
        4 => {
            let value = reader.read_i64::<BigEndian>()?;

            Ok(Tag::Long(value))
        }
        5 => {
            let value = reader.read_f32::<BigEndian>()?;

            Ok(Tag::Float(value))
        }
        6 => {
            let value = reader.read_f64::<BigEndian>()?;

            Ok(Tag::Double(value))
        }
        7 => {
            let length = reader.read_u32::<BigEndian>()?;
            let mut value = Vec::new();

            for _ in 0..length {
                value.push(reader.read_i8()?);
            }

            Ok(Tag::ByteArray(value))
        }
        8 => {
            let value = read_string(reader)?;

            Ok(Tag::String(value))
        }
        9 => {
            let list_tags_id = reader.read_u8()?;
            let length = reader.read_u32::<BigEndian>()?;
            let mut value = Vec::new();

            for _ in 0..length {
                value.push(read_tag(list_tags_id, None, reader)?);
            }

            Ok(Tag::List(value))
        }
        10 => {
            let mut tags = LinkedHashMap::new();

            loop {
                let tag_id = reader.read_u8()?;

                // Compound tag end reached.
                if tag_id == 0 {
                    break;
                }

                let name = read_string(reader)?;
                let tag = read_tag(tag_id, Some(name.as_str()), reader)?;

                tags.insert(name, tag);
            }

            let compound_tag = CompoundTag {
                name: name.map(|s| s.into()),
                tags,
            };

            Ok(Tag::Compound(compound_tag))
        }
        11 => {
            let length = reader.read_u32::<BigEndian>()?;
            let mut value = Vec::new();

            for _ in 0..length {
                value.push(reader.read_i32::<BigEndian>()?);
            }

            Ok(Tag::IntArray(value))
        }
        12 => {
            let length = reader.read_u32::<BigEndian>()?;
            let mut value = Vec::new();

            for _ in 0..length {
                value.push(reader.read_i64::<BigEndian>()?);
            }

            Ok(Tag::LongArray(value))
        }
        tag_type_id => Err(TagDecodeError::UnknownTagType { tag_type_id }),
    }
}

fn read_string<R: Read>(reader: &mut R) -> Result<String, TagDecodeError> {
    let length = reader.read_u16::<BigEndian>()?;
    let mut buf = vec![0; length as usize];
    reader.read_exact(&mut buf)?;

    Ok(String::from_utf8_lossy(&buf).into_owned())
}
