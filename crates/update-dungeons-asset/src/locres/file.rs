use std::collections::hash_map::IntoValues;
use std::collections::HashMap;

use encoding_rs::UTF_16LE;
use winnow::bytes::{tag, take};
use winnow::combinator::{cond, opt};
use winnow::multi::count;
use winnow::number::{le_i32, le_u32, le_u64, le_u8};
use winnow::sequence::{preceded, terminated};
use winnow::Parser;

use super::version::LocresVersion;
use super::LocresNamespace;

const MAGIC: &[u8] = &[
    0x0E, 0x14, 0x74, 0x75, 0x67, 0x4A, 0x03, 0xFC, 0x4A, 0x15, 0x90, 0x9D, 0xC3, 0x37, 0x7F, 0x1B,
];

pub struct LocresFile {
    version: LocresVersion,
    map: HashMap<String, LocresNamespace>,
}

impl LocresFile {
    pub fn into_values(self) -> IntoValues<String, LocresNamespace> {
        self.map.into_values()
    }
}

pub type ParseResult<'a, O> = winnow::IResult<&'a [u8], O>;

fn parse_unreal_string(s: &[u8]) -> ParseResult<String> {
    let (s, length) = le_i32.parse_next(s)?;
    if length >= 0 {
        // ASCII
        take(usize::try_from(length).unwrap())
            .map(String::from_utf8_lossy)
            .map(|res| res.trim_end_matches('\0').to_string())
            .parse_next(s)
    } else {
        // UTF-16 LE
        take(usize::try_from(length * -2).unwrap())
            .map(|unicode: &[u8]| UTF_16LE.decode(unicode).0)
            .map(|res| res.trim_end_matches('\0').to_string())
            .parse_next(s)
    }
}

impl LocresFile {
    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn read(s: &[u8]) -> ParseResult<LocresFile> {
        let root = s;

        let (s, version) =
            opt(preceded(tag(MAGIC), le_u8).map_res(LocresVersion::try_from)).parse_next(s)?;
        let version = version.unwrap_or(LocresVersion::Legacy);

        let (s, localized_string_arraay) =
            cond(version >= LocresVersion::Compact, |s| -> ParseResult<_> {
                let (s, offset) = le_u64(s)?;
                let recover_point = s;
                let (s, _) = take(offset).parse_next(root)?;
                let (s, localized_string_count) = le_u32.map_res(usize::try_from).parse_next(s)?;

                let (_, localized_string_arraay): (_, Vec<_>) = count(
                    terminated(
                        parse_unreal_string,
                        cond(version >= LocresVersion::Optimized, le_i32),
                    ),
                    localized_string_count,
                )
                .parse_next(s)?;

                Ok((recover_point, localized_string_arraay))
            })
            .parse_next(s)?;
        let localized_string_array = localized_string_arraay.unwrap_or_default();

        let (s, _) = cond(version >= LocresVersion::Optimized, le_i32).parse_next(s)?; // entriesCount

        let (s, namespace_count) = le_i32.map_res(usize::try_from).parse_next(s)?;
        let mut namespaces = HashMap::with_capacity(namespace_count);

        let (s, entries): (_, Vec<_>) = count(
            |s| {
                let (s, _) = cond(version >= LocresVersion::Optimized, le_i32).parse_next(s)?; // namespaceKeyHash

                let (s, namespace_key) = parse_unreal_string(s)?;
                let (s, key_count) = le_i32.map_res(usize::try_from).parse_next(s)?;
                let mut ns = LocresNamespace::new(namespace_key.clone());

                let (s, entries): (_, Vec<_>) = count(
                    |s| {
                        let (s, _) =
                            cond(version >= LocresVersion::Optimized, le_u32).parse_next(s)?; // string_key_hash
                        let (s, string_key) = parse_unreal_string(s)?;
                        let (s, _) = le_u32.parse_next(s)?; // source_string_hash

                        let (s, localized_string) = if version >= LocresVersion::Compact {
                            le_i32
                                .map_res(usize::try_from)
                                .map(|i| localized_string_array[i].clone())
                                .parse_next(s)?
                        } else {
                            parse_unreal_string(s)?
                        };

                        Ok((s, (string_key, localized_string)))
                    },
                    key_count,
                )
                .parse_next(s)?;

                for (k, v) in entries {
                    ns.insert(k, v);
                }

                Ok((s, (namespace_key, ns)))
            },
            namespace_count,
        )
        .parse_next(s)?;

        for (k, v) in entries {
            namespaces.insert(k, v);
        }

        Ok((
            s,
            LocresFile {
                version,
                map: namespaces,
            },
        ))
    }
}
