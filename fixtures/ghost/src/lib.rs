verus! {
fn signature(x: u64) -> (r: u64)
    requires
        x < 10,
    ensures r == x + 1,
    recommends x < 20,
    decreases x,
{
    x + 1
}
fn loops(mut x: u64) {
    while x > 0
        invariant
            x <= 10,
        invariant_except_break x <= 11,
        decreases x,
        ensures x == 0,
    {
        x -= 1;
    }
    loop
        invariant x == 0,
    {
        break;
    }
    for y in 0..x
        invariant y <= x,
    {
        consume(y);
    }
}
fn statements(x: u64) {
    proof {
        lemma();
    }
    assert(x > 0);
    assert(x > 0) by {
        lemma();
    }
    let y = x;
    assert forall |i: int| i == i by {
        lemma();
    }
    assume(x < 10);
    let ghost a =
        x;
    let tracked b = x;
    let c = x; assert(c > 0);
    consume(y);
}
fn mixed(x: u64) requires x > 0, { assert(x > 0); consume(x); }
fn opaque() {
    other! {
        assert(false);
    }
    consume(1);
}
fn literals() {
    let ghost text = r#"first

    // literal text
    "#;
    consume(1);
}
fn comments() {
    // assert(fake);
    let text = "assert(fake);";
    /* requires fake */
    consume(text);
}
}
fn ordinary() {
    assert(true);
    let ghost = 1;
}
