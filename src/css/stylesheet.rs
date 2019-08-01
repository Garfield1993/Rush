enum Selector {
    Basic(BasicSelector),
}

pub struct BasicSelector {
    tagName: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}

pub struct Declaration {
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
pub struct Stylesheet {
    rules: Vec<Rule>,
}

pub struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}


pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        // http://www.w3.org/TR/selectors/#specificity
        let Selector::Basic(ref basic) = *self;
        let a = basic.id.iter().count();
        let b = basic.class.len();
        let c = basic.tagName.iter().count();
        (a, b, c)
    }
}
