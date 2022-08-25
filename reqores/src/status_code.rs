#[derive(PartialEq)]
#[repr(u16)]
pub enum StatusCode {
    Ok = 200,
    NoContent = 204,
    BadRequest = 400,
    Forbidden = 403,
    Notfound = 404,
}
