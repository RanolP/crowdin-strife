#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum LocresVersion {
    Legacy = 0,
    Compact = 1,
    Optimized = 2,
    #[allow(non_camel_case_types)]
    Optimized_CityHash64_UTF16 = 3,
}

#[derive(Debug)]
pub struct VersionNotFound;

impl TryFrom<u8> for LocresVersion {
    type Error = VersionNotFound;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LocresVersion::Legacy),
            1 => Ok(LocresVersion::Compact),
            2 => Ok(LocresVersion::Optimized),
            3 => Ok(LocresVersion::Optimized_CityHash64_UTF16),
            _ => Err(VersionNotFound),
        }
    }
}
