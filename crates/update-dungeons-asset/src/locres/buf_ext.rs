use std::{array::TryFromSliceError, cmp::Ordering, string::FromUtf8Error};

pub trait BufExt {
    fn seek_u8(&self) -> Result<u8, SeekReadError>;
    fn seek_u32(&self) -> Result<u32, SeekReadError>;
    fn seek_i32(&self) -> Result<i32, SeekReadError>;
    fn seek_i64(&self) -> Result<i64, SeekReadError>;
    fn seek_u64(&self) -> Result<u64, SeekReadError>;
}
pub trait BufMutExt {
    fn read_u8(&mut self) -> Result<u8, SeekReadError>;
    fn read_u32(&mut self) -> Result<u32, SeekReadError>;
    fn read_i32(&mut self) -> Result<i32, SeekReadError>;
    fn read_i64(&mut self) -> Result<i64, SeekReadError>;
    fn read_u64(&mut self) -> Result<u64, SeekReadError>;
    fn read_unreal_string(&mut self) -> Result<String, SeekReadError>;
}

#[derive(Debug)]
pub enum SeekReadError {
    Slice(TryFromSliceError),
    Utf8(FromUtf8Error),
}

impl From<TryFromSliceError> for SeekReadError {
    fn from(value: TryFromSliceError) -> Self {
        SeekReadError::Slice(value)
    }
}

impl From<FromUtf8Error> for SeekReadError {
    fn from(value: FromUtf8Error) -> Self {
        SeekReadError::Utf8(value)
    }
}

impl BufExt for &[u8] {
    fn seek_u8(&self) -> Result<u8, SeekReadError> {
        let res = self[0];
        Ok(res)
    }

    fn seek_u32(&self) -> Result<u32, SeekReadError> {
        let res = u32::from_be_bytes(self[0..4].try_into()?);
        Ok(res)
    }

    fn seek_i32(&self) -> Result<i32, SeekReadError> {
        let res = i32::from_be_bytes(self[0..4].try_into()?);
        Ok(res)
    }

    fn seek_i64(&self) -> Result<i64, SeekReadError> {
        let res = i64::from_be_bytes(self[0..8].try_into()?);
        Ok(res)
    }

    fn seek_u64(&self) -> Result<u64, SeekReadError> {
        let res = u64::from_be_bytes(self[0..8].try_into()?);
        Ok(res)
    }
}

impl BufMutExt for &[u8] {
    fn read_u8(&mut self) -> Result<u8, SeekReadError> {
        let res = self.seek_u8()?;
        *self = &self[1..];
        Ok(res)
    }

    fn read_u32(&mut self) -> Result<u32, SeekReadError> {
        let res = self.seek_u32()?;
        *self = &self[4..];
        Ok(res)
    }

    fn read_i32(&mut self) -> Result<i32, SeekReadError> {
        let res = self.seek_i32()?;
        *self = &self[4..];
        Ok(res)
    }

    fn read_i64(&mut self) -> Result<i64, SeekReadError> {
        let res = self.seek_i64()?;
        *self = &self[8..];
        Ok(res)
    }

    fn read_u64(&mut self) -> Result<u64, SeekReadError> {
        let res = self.seek_u64()?;
        *self = &self[8..];
        Ok(res)
    }

    fn read_unreal_string(&mut self) -> Result<String, SeekReadError> {
        let length = self.read_i32()?;
        let result = match length.cmp(&0) {
            Ordering::Greater => {
                // ASCII
                let res = String::from_utf8(self[0..length as usize].to_vec())?;
                *self = &self[length as usize..];
                res
            }
            Ordering::Equal => {
                // length == 0
                return Ok(String::new());
            }
            Ordering::Less => {
                // Unicode
                let res = String::from_utf8(self[0..(length * -2) as usize].to_vec())?;
                *self = &self[(length * -2) as usize..];
                res
            }
        };
        Ok(result.trim_end_matches('\0').to_string())
    }
}
