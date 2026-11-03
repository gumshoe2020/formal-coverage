pub fn clean(x: u32) -> u32 { x }
pub fn parse(x: u32) -> u32 { x }
#[cfg_attr(hax, hax_lib::ensures(|result| parse(result) == x))]
pub fn round_trip(x: u32) -> u32 { x }
