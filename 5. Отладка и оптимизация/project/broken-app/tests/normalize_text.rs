use broken_app::normalize;

#[test]
fn normalize_simple() {
    assert_eq!(normalize(" Hello World "), "helloworld");
}

#[test]
fn normalize_tabs() {
    assert_eq!(normalize("		Hello	World	"), "helloworld");
}

#[test]
fn normalize_many_space() {
    assert_eq!(normalize("    Hello    World     "), "helloworld");
}

#[test]
fn normalize_others() {
    assert_eq!(normalize(" Hello World "), "helloworld");
}
