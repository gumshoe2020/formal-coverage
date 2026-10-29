verus! {
    pub fn contracted(x: u64) -> (r: u64)
        requires x < 10,
        ensures r == x + 1,
    { x + 1 }
}
fn ordinary() {}
fn untouched() {}
#[test]
fn example() { ordinary(); }
#[kani::proof]
fn harness() { ordinary(); }

verus! {
    fn match_contract(x: Option<u64>) -> (r: u64)
        ensures match x { Some(n) => r == n, None => r == 0, },
    {
        let value = match x { Some(n) => n, None => 0 };
        value
    }
}
