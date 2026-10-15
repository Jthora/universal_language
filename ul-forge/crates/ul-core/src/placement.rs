//! The placement channel: handedness, which a combinatorial map provably cannot carry.
//!
//! WHY THIS MODULE EXISTS (notes 076, 077, 078)
//!   `PLACEMENT-HANDEDNESS-IS-INVISIBLE-TO-THE-MAP` is VERIFIED: A-beside-B and B-beside-A give
//!   the SAME map, so a corpus entry — which is a map plus its expected invariants — cannot
//!   express a distinction carried by handedness. Note 076 measured the size of that hole: of the
//!   180 attested (operator, a, b) cells, **85 get their handedness from outside the map**, 55
//!   from the alphabet and 30 from arrangement. This is the second channel it specified.
//!
//!   Note 078 then found the channel's data already in the vendored register, as a nine-parameter
//!   configuration space in which `reflect-B` is a first-class two-valued axis.
//!
//! WHAT IT IS AND IS NOT
//!   It computes a handedness SOURCE from the composition — which mark on each side, under which
//!   operator — and nothing else. It does not render, does not decide what a figure means, and
//!   does not touch the map. The map and this channel are deliberately disjoint: that is the whole
//!   point of a second channel, and anything that made them overlap would reintroduce the hole.
//!
//! THE ONE BIT THE ALPHABET SUPPLIES
//!   A plane figure is achiral iff its symmetry group contains a reflection — iff the group is
//!   dihedral or orthogonal rather than cyclic. Of the six marks exactly one group is cyclic, so
//!   **Wave is the unique chiral letter**, and it is the smooth C2 one: D2 contains C2, so the
//!   Line's half-turn is also realised by a reflection, while reflecting a sine in any line
//!   returns a half-period shift and never the sine.
//!
//!   The table below is a const, and `tests/placement_tests.rs` asserts it against the register's
//!   `enantiomorphs` sheet in both directions — the same drift guard `schema_agreement_tests.rs`
//!   installs for the GIR schema, and for the same reason: a hardcoded table that nothing compares
//!   to its source is a number that goes stale silently (`COUNTS-ARE-GENERATED`).

/// The number of mirror axes a figure has. `Infinite` is O(2); a map cannot represent it at all
/// (`MAP-SYMMETRY-BOUNDS-DRAWN-SYMMETRY`'s scope), which is why this is not a `u32`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MirrorAxes {
    /// Every line through the centre is an axis — the O(2) marks.
    Infinite,
    /// Exactly this many axes. Zero means chiral.
    Finite(u32),
}

impl MirrorAxes {
    /// True when the figure has at least `n` mirror axes.
    pub fn at_least(self, n: u32) -> bool {
        match self {
            MirrorAxes::Infinite => true,
            MirrorAxes::Finite(k) => k >= n,
        }
    }

    /// No axis at all: the figure is not superposable on its mirror image.
    pub fn chiral(self) -> bool {
        self == MirrorAxes::Finite(0)
    }
}

/// One of the six marks. The six are the complete 3 x 2 product of rotation class by curvature
/// class (`MARKS-ARE-A-COMPLETE-PRODUCT`), and handedness is a single cell of that grid — so it is
/// a function of neither coordinate alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mark {
    Point,
    Circle,
    Line,
    Wave,
    Angle,
    Curve,
}

impl Mark {
    /// Every mark, in the register's own row order.
    pub const ALL: [Mark; 6] = [
        Mark::Point,
        Mark::Line,
        Mark::Angle,
        Mark::Curve,
        Mark::Wave,
        Mark::Circle,
    ];

    /// The symmetry group, as the register's `enantiomorphs` sheet names it.
    pub fn group(self) -> &'static str {
        match self {
            Mark::Point | Mark::Circle => "O(2)",
            Mark::Line => "D2",
            Mark::Wave => "C2",
            Mark::Angle | Mark::Curve => "D1",
        }
    }

    /// Mirror axes, derived from the group rather than listed separately: O(2) has every axis,
    /// D_n has n, C_n has none.
    pub fn mirror_axes(self) -> MirrorAxes {
        match self.group() {
            "O(2)" => MirrorAxes::Infinite,
            "D2" => MirrorAxes::Finite(2),
            "D1" => MirrorAxes::Finite(1),
            "C2" => MirrorAxes::Finite(0),
            g => unreachable!("mark {self:?} has unhandled group {g}"),
        }
    }

    /// A chiral mark is one whose group contains no reflection.
    pub fn is_chiral(self) -> bool {
        self.mirror_axes().chiral()
    }

    /// Exactly one mirror axis — the C1/D1 pair, which is what the arrangement rule turns on.
    pub fn is_modulating(self) -> bool {
        self.mirror_axes() == MirrorAxes::Finite(1)
    }

    pub fn name(self) -> &'static str {
        match self {
            Mark::Point => "Point",
            Mark::Circle => "Circle",
            Mark::Line => "Line",
            Mark::Wave => "Wave",
            Mark::Angle => "Angle",
            Mark::Curve => "Curve",
        }
    }

    pub fn from_name(s: &str) -> Option<Mark> {
        Mark::ALL.into_iter().find(|m| m.name() == s)
    }
}

/// What the placement leaves free, which is what decides whether arrangement can manufacture
/// handedness. Grouped from the five attested binary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorClass {
    /// `nest`, `superpose_concentric` — separation 0. The centre is shared and relative rotation
    /// is free, so each mark's mirror axis can be rotated onto the other's and a common axis
    /// always exists when both marks have one. This row has an argument, not a fit.
    Concentric,
    /// `cross` — the marks pass through one another.
    Crossing,
    /// `concatenate`, `superpose_displaced` — separation is nonzero, so the line through the two
    /// centres is distinguished.
    Displacing,
}

impl OperatorClass {
    pub fn of(operator: &str) -> Option<OperatorClass> {
        match operator {
            "nest" | "superpose_concentric" => Some(OperatorClass::Concentric),
            "cross" => Some(OperatorClass::Crossing),
            "concatenate" | "superpose_displaced" => Some(OperatorClass::Displacing),
            _ => None,
        }
    }
}

/// Where a composite figure's handedness comes from. The register's own three-way attribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Handedness {
    /// Superposable on its mirror image.
    Achiral,
    /// An operand is itself chiral, so the composite is. Derived from one bit of the mark table
    /// and exact on all 55 attested cells (`INHERITED-HANDEDNESS-IS-THE-WAVE`, VERIFIED).
    Inherited,
    /// Every operand is achiral and the arrangement is not. This branch rests on
    /// `ARRANGEMENT-RULE-IS-FITTED`, which is ARGUED, not VERIFIED — see that claim's scope before
    /// relying on it, and expect any corpus entry that reaches this branch to be capped at ARGUED.
    Arrangement,
}

impl Handedness {
    pub fn chiral(self) -> bool {
        self != Handedness::Achiral
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Handedness::Achiral => "achiral",
            Handedness::Inherited => "inherited",
            Handedness::Arrangement => "arrangement",
        }
    }
}

/// A binary composition's placement, carrying only what handedness depends on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Placement {
    pub operator: String,
    pub a: Mark,
    pub b: Mark,
    /// Whether operand B is drawn mirrored. This selects WHICH enantiomorph the figure is; it does
    /// not change whether the figure HAS enantiomorphs, so `handedness` does not read it. What it
    /// does change is whether the configuration realises an attested operator at all — see
    /// `survives_reflection`.
    pub reflect_b: bool,
}

impl Placement {
    pub fn new(operator: impl Into<String>, a: Mark, b: Mark) -> Self {
        Placement {
            operator: operator.into(),
            a,
            b,
            reflect_b: false,
        }
    }

    pub fn mirrored(mut self) -> Self {
        self.reflect_b = !self.reflect_b;
        self
    }

    /// Where this composition's handedness comes from.
    ///
    /// `None` when the operator is outside the five attested binary ones — a unary composition
    /// (`close`, `reflect_pair`) has no second operand and this channel says nothing about it.
    /// Returning `None` rather than guessing is deliberate: 7 of the 93 orbits are generated
    /// unarily and carry no cell in the attested handedness column either.
    pub fn handedness(&self) -> Option<Handedness> {
        let class = OperatorClass::of(&self.operator)?;

        // The alphabet's one bit, first: it needs no operator and has no free parameter.
        if self.a.is_chiral() || self.b.is_chiral() {
            return Some(Handedness::Inherited);
        }
        // Congruent operands leave the perpendicular bisector available as an axis.
        if self.a == self.b {
            return Some(Handedness::Achiral);
        }

        let modulating = [self.a, self.b].iter().filter(|m| m.is_modulating()).count();
        Some(match class {
            OperatorClass::Concentric => Handedness::Achiral,
            OperatorClass::Crossing if modulating == 2 => Handedness::Arrangement,
            OperatorClass::Crossing => Handedness::Achiral,
            OperatorClass::Displacing if modulating >= 1 => Handedness::Arrangement,
            OperatorClass::Displacing => Handedness::Achiral,
        })
    }

    /// Whether mirroring operand B leaves the configuration inside the attested operator
    /// vocabulary.
    ///
    /// Measured over the register's reflect-B pairs: survival holds exactly when B has two or more
    /// mirror axes — 16 informative pairs, 16 agreements, 0 disagreements
    /// (`OPERATOR-SURVIVAL-UNDER-REFLECTION-TRACKS-SYMMETRY`). The mechanism is immediate: an O(2)
    /// mark is unmoved by a mirror and the Line maps to itself, so the configuration is literally
    /// unchanged, while Wave and Angle/Curve genuinely change.
    ///
    /// This is what makes the channel well-formedness-bearing rather than decorative: flipping the
    /// bit can take a composition OUT of the language.
    pub fn survives_reflection(&self) -> bool {
        self.b.mirror_axes().at_least(2)
    }
}
