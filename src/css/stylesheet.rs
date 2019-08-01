enum Selector {
    Basic(BasicSelector),
}

struct BasicSelector {
    tagName: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}

struct Declaration {
    name: String,
    value: Value,
}

enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
    // insert more values here
}

enum Unit {
    Px,
    // TODO: more units
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}


/**
 * CSS style sheet and rule
 */
struct Stylesheet {
    rules: Vec<Rule>,
}

struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}