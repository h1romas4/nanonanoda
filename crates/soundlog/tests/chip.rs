use soundlog::VgmBuilder;
use soundlog::chip::*;

#[test]
fn add_chip_accepts_tag_only_chip() {
    // Ensure we can pass a tag-only `Chip` to `add_chip` without a Spec payload.
    let mut b = VgmBuilder::new();
    b.register_chip(Chip::Ym2612, 0, 8000000);
    let doc = b.finalize();
    assert_eq!(doc.header.ym2612_clock, 8000000);
}
