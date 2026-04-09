//! Abstract Syntax Tree types for parsed UL-Script.

/// A parsed UL-Script document containing one or more glyph definitions.
#[derive(Debug, Clone, PartialEq)]
pub struct AstDocument {
    pub glyphs: Vec<AstGlyph>,
}

/// A single glyph definition (top-level composition or comment).
#[derive(Debug, Clone, PartialEq)]
pub enum AstGlyph {
    Composition(AstComposition),
    Comment(String),
}

/// A sequence of terms connected by operators.
#[derive(Debug, Clone, PartialEq)]
pub struct AstComposition {
    /// The first term and then (operator, term) pairs.
    pub head: Box<AstTerm>,
    pub tail: Vec<(AstOperator, AstTerm)>,
}

/// A single term: either a mark, an assertion modifier, or a parenthesized group.
#[derive(Debug, Clone, PartialEq)]
pub enum AstTerm {
    Mark(AstMark),
    Group(AstComposition),
    /// `?{...}` evidential, `!{...}` emphatic, `~?{...}` hedged
    AssertionModifier {
        kind: AstAssertionModifierKind,
        content: AstComposition,
    },
    /// `[]{...}` necessity, `<>{...}` possibility
    ModalUnary {
        kind: AstModalKind,
        content: AstComposition,
    },
    /// `[]->{antecedent}{consequent}` counterfactual
    ModalCounterfactual {
        antecedent: AstComposition,
        consequent: AstComposition,
    },
    /// `assert{...}`, `query{...}`, etc. — force annotation
    ForceAnnotation {
        force: AstForceKind,
        content: AstComposition,
    },
}

/// A geometric primitive mark with optional nested content.
#[derive(Debug, Clone, PartialEq)]
pub struct AstMark {
    pub primitive: AstPrimitive,
    pub content: Option<AstComposition>,
}

/// A geometric primitive.
#[derive(Debug, Clone, PartialEq)]
pub enum AstPrimitive {
    Point,
    Circle,
    Triangle,
    Square,
    Pentagon,
    Hexagon,
    RightArrow,
    LeftArrow,
    BiArrow,
    Curve,
    Angle(f64),
    /// `○_x` — variable slot for binding
    VariableSlot(String),
    /// `●_x` — bound reference to a variable
    BoundRef(String),
}

/// An operator connecting two terms.
#[derive(Debug, Clone, PartialEq)]
pub enum AstOperator {
    /// `→`, `←`, or `↔` with optional angle
    Connection {
        direction: AstDirection,
        angle: Option<f64>,
    },
    /// `|`
    Adjacency,
    /// `&`
    Intersection,
}

/// Direction of a connection.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AstDirection {
    Right,
    Left,
    Both,
}

/// Kind of assertion-level modification.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AstAssertionModifierKind {
    Evidential,
    Emphatic,
    Hedged,
}

/// Kind of modal operator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AstModalKind {
    Necessity,
    Possibility,
}

/// Kind of performative force.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AstForceKind {
    Assert,
    Query,
    Direct,
    Commit,
    Express,
    Declare,
}
