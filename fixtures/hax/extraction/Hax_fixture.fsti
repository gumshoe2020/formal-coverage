module Hax_fixture
val clean (x: u32) : Tot u32
val parse (x: u32) : Tot u32
val round_trip (x: u32) : Pure u32 True (ensures fun result -> parse result == x)
