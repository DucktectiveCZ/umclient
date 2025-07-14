use crate::proto772::encoding::Error;

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum State {
    Status = 1,
    Login = 2,
    Transfer = 3,
}

impl TryFrom<i32> for State {
    type Error = Error;

    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Status),
            2 => Ok(Self::Login),
            3 => Ok(Self::Transfer),
            v => Err(Error::BadState(v)),
        }
    }
}
