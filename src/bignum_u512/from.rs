use super::u512;

// Constructors from standard integer types
impl From<u128> for u512 {
    fn from(value: u128) -> u512 {
        return u512 { data: [value as u64, (value >> 64) as u64, 0, 0, 0, 0, 0, 0] };
    }
}

impl From<u8>    for u512 { fn from(value: u8) -> u512    { u512::from(value as u128) } }
impl From<&u8>   for u512 { fn from(value: &u8) -> u512   { u512::from(*value as u128) } }
impl From<u16>   for u512 { fn from(value: u16) -> u512   { u512::from(value as u128) } }
impl From<&u16>  for u512 { fn from(value: &u16) -> u512  { u512::from(*value as u128) } }
impl From<u32>   for u512 { fn from(value: u32) -> u512   { u512::from(value as u128) } }
impl From<&u32>  for u512 { fn from(value: &u32) -> u512  { u512::from(*value as u128) } }
impl From<u64>   for u512 { fn from(value: u64) -> u512   { u512::from(value as u128) } }
impl From<&u64>  for u512 { fn from(value: &u64) -> u512  { u512::from(*value as u128) } }
impl From<&u128> for u512 { fn from(value: &u128) -> u512 { u512::from(*value) } }

impl From<i128> for u512 {
    fn from(value: i128) -> u512 {
        if value < 0 {
            panic!("Tried to create u512 from negative value!");
        }

        return u512::from(value as u128);
    }
}

impl From<i8>    for u512 { fn from(value: i8) -> u512    { u512::from(value as i128) } }
impl From<&i8>   for u512 { fn from(value: &i8) -> u512   { u512::from(*value as i128) } }
impl From<i16>   for u512 { fn from(value: i16) -> u512   { u512::from(value as i128) } }
impl From<&i16>  for u512 { fn from(value: &i16) -> u512  { u512::from(*value as i128) } }
impl From<i32>   for u512 { fn from(value: i32) -> u512   { u512::from(value as i128) } }
impl From<&i32>  for u512 { fn from(value: &i32) -> u512  { u512::from(*value as i128) } }
impl From<i64>   for u512 { fn from(value: i64) -> u512   { u512::from(value as i128) } }
impl From<&i64>  for u512 { fn from(value: &i64) -> u512  { u512::from(*value as i128) } }
impl From<&i128> for u512 { fn from(value: &i128) -> u512 { u512::from(*value) } }
