// excluded comment
pub fn entry() {
    helper();
}
fn helper() {}
mod nested {
    pub fn other() {
        // comment excluded
        let _x = 1;
    }
}
verus! {
    pub fn checked(x: u64) -> (r: u64)
        requires x < 10,
        ensures r == x + 1,
    {
        x + 1
    }
}
