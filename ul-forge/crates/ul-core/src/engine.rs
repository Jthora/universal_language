//! The M2 engine, v1 — interaction-net reduction over port-graphs whose agents are rotations
//! with a marked dart (spec/engine-core.md; research/notes/051, 057).
//!
//! Meaning as behavior: a program's meaning is what it does under local rewriting, and the only
//! rewrites are on ACTIVE PAIRS — two agents connected principal-to-principal (Lafont 1990, via
//! Gay 1991: "two agents can only interact if they are connected by their principal ports").
//!
//! The linear discipline is enforced STRUCTURALLY, not by convention: rule registration rejects
//! any right-hand side that drops or duplicates a wire (`Rule::validate`). Copying and deleting
//! exist only as explicit agents a system defines — the no-free-copy / no-free-delete baseline
//! that `claims.yaml#QUANTUM-IS-LINEAR-DISCIPLINE` records.
//!
//! Every net exports to the grammar substrate (`Net::to_map`): agents become vertices whose
//! rotation is the port order with the principal first — exactly Gay's serialization of the
//! geometric arrangement — so engine states are checkable by the same validity rules as any
//! other expression.

use crate::map::{CombinatorialMap, Dart};
use std::collections::HashMap;

/// Index into the symbol table.
pub type SymbolId = usize;
/// A port address: `(agent, slot)`, slot 0 being the principal port.
pub type Port = (usize, usize);

/// The reserved interface symbol: arity 0, no rules may be registered on it. Free ports of a net
/// are FREE agents, so the wiring is total and external handles survive rewriting.
pub const FREE: SymbolId = 0;

/// A system: the symbol table and the interaction rules.
#[derive(Debug, Default)]
pub struct System {
    names: Vec<String>,
    /// Auxiliary arity (the principal port is not counted).
    arity: Vec<usize>,
    rules: HashMap<(SymbolId, SymbolId), Rule>,
}

/// One side of a rule's right-hand-side connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RPort {
    /// Index into the freed interface: the LHS pair's auxiliary ports, first agent's in order,
    /// then the second agent's.
    Iface(usize),
    /// `(rhs agent index, slot)` — slot 0 is the principal.
    New(usize, usize),
}

/// An interaction rule for an active pair `(a, b)`: the replacement agents and the complete
/// rewiring. Linearity is checked at registration: every interface index and every slot of every
/// replacement agent must be used exactly once.
#[derive(Debug, Clone)]
pub struct Rule {
    pub rhs_agents: Vec<SymbolId>,
    pub links: Vec<(RPort, RPort)>,
}

impl System {
    pub fn new() -> Self {
        let mut s = System::default();
        s.names.push("FREE".into());
        s.arity.push(0);
        s
    }

    /// Declare a symbol with the given auxiliary arity.
    pub fn symbol(&mut self, name: &str, aux_arity: usize) -> SymbolId {
        self.names.push(name.into());
        self.arity.push(aux_arity);
        self.names.len() - 1
    }

    pub fn arity(&self, s: SymbolId) -> usize {
        self.arity[s]
    }

    /// Register the rule for the active pair `(a, b)`. Rejects rules on FREE and any RHS that
    /// violates linearity — dropping a wire (free delete) or using one twice (free copy).
    pub fn rule(&mut self, a: SymbolId, b: SymbolId, rule: Rule) -> Result<(), String> {
        if a == FREE || b == FREE {
            return Err("FREE is an interface, not an agent: no rules".into());
        }
        let iface_total = self.arity[a] + self.arity[b];
        let mut iface_used = vec![0usize; iface_total];
        let mut slot_used: Vec<Vec<usize>> = rule
            .rhs_agents
            .iter()
            .map(|&s| vec![0usize; self.arity[s] + 1])
            .collect();
        for (p, q) in &rule.links {
            for side in [p, q] {
                match *side {
                    RPort::Iface(i) => {
                        if i >= iface_total {
                            return Err(format!("interface index {i} out of range"));
                        }
                        iface_used[i] += 1;
                    }
                    RPort::New(t, s) => {
                        if t >= slot_used.len() || s >= slot_used[t].len() {
                            return Err(format!("rhs port ({t},{s}) out of range"));
                        }
                        slot_used[t][s] += 1;
                    }
                }
            }
        }
        for (i, &n) in iface_used.iter().enumerate() {
            if n == 0 {
                return Err(format!("linearity: interface {i} dropped — no free delete"));
            }
            if n > 1 {
                return Err(format!(
                    "linearity: interface {i} used {n} times — no free copy"
                ));
            }
        }
        for (t, slots) in slot_used.iter().enumerate() {
            for (s, &n) in slots.iter().enumerate() {
                if n != 1 {
                    return Err(format!("linearity: rhs port ({t},{s}) used {n} times"));
                }
            }
        }
        self.rules.insert((a, b), rule);
        Ok(())
    }
}

/// A net: agents (tombstoned on rewrite, ids stable) and a total symmetric wiring.
#[derive(Debug, Default)]
pub struct Net {
    agents: Vec<Option<SymbolId>>,
    partner: HashMap<Port, Port>,
}

impl Net {
    pub fn new() -> Self {
        Net::default()
    }

    pub fn add(&mut self, symbol: SymbolId) -> usize {
        self.agents.push(Some(symbol));
        self.agents.len() - 1
    }

    /// A FREE agent: a stable external handle. Its single port is slot 0.
    pub fn free_port(&mut self) -> usize {
        self.add(FREE)
    }

    pub fn wire(&mut self, p: Port, q: Port) {
        self.partner.insert(p, q);
        self.partner.insert(q, p);
    }

    pub fn symbol_of(&self, agent: usize) -> Option<SymbolId> {
        self.agents.get(agent).copied().flatten()
    }

    pub fn live_agents(&self) -> usize {
        self.agents.iter().flatten().filter(|&&s| s != FREE).count()
    }

    /// Active pairs: wires joining two principal ports of live, non-FREE agents.
    pub fn active_pairs(&self, sys: &System) -> Vec<(usize, usize)> {
        let mut out = Vec::new();
        for (&(a, sa), &(b, sb)) in &self.partner {
            if sa == 0 && sb == 0 && a < b {
                if let (Some(x), Some(y)) = (self.symbol_of(a), self.symbol_of(b)) {
                    if x != FREE
                        && y != FREE
                        && (sys.rules.contains_key(&(x, y)) || sys.rules.contains_key(&(y, x)))
                    {
                        out.push((a, b));
                    }
                }
            }
        }
        out.sort_unstable();
        out
    }

    /// Apply the registered rule to the active pair `(a, b)`. Local: only the pair and its
    /// immediate wiring change.
    pub fn apply(&mut self, sys: &System, a: usize, b: usize) -> Result<(), String> {
        let (sa, sb) = (
            self.symbol_of(a).ok_or("dead agent")?,
            self.symbol_of(b).ok_or("dead agent")?,
        );
        // orientation: interface = first agent's aux then second's, per the registered order
        let (first, second, rule) = if let Some(r) = sys.rules.get(&(sa, sb)) {
            (a, b, r.clone())
        } else if let Some(r) = sys.rules.get(&(sb, sa)) {
            (b, a, r.clone())
        } else {
            return Err(format!(
                "no rule for ({}, {})",
                sys.names[sa], sys.names[sb]
            ));
        };
        let (sf, ss) = (
            self.symbol_of(first).unwrap(),
            self.symbol_of(second).unwrap(),
        );

        // the freed interface: what each auxiliary port was wired to
        let mut iface: Vec<Port> = Vec::new();
        for s in 1..=sys.arity(sf) {
            iface.push(self.partner[&(first, s)]);
        }
        for s in 1..=sys.arity(ss) {
            iface.push(self.partner[&(second, s)]);
        }

        // remove the pair
        for agent in [first, second] {
            let ar = sys.arity(self.symbol_of(agent).unwrap());
            for s in 0..=ar {
                if let Some(p) = self.partner.remove(&(agent, s)) {
                    self.partner.remove(&p);
                }
            }
            self.agents[agent] = None;
        }

        // instantiate the right-hand side
        let fresh: Vec<usize> = rule.rhs_agents.iter().map(|&s| self.add(s)).collect();
        let resolve = |rp: RPort, iface: &[Port], fresh: &[usize]| -> Port {
            match rp {
                RPort::Iface(i) => iface[i],
                RPort::New(t, s) => (fresh[t], s),
            }
        };
        for (p, q) in &rule.links {
            let pp = resolve(*p, &iface, &fresh);
            let qq = resolve(*q, &iface, &fresh);
            self.wire(pp, qq);
        }
        Ok(())
    }

    /// Reduce to normal form. Returns the number of interactions. Strong confluence makes the
    /// scan order irrelevant to the result — `reduction_order_does_not_change_the_answer` checks
    /// this rather than assumes it.
    pub fn normalize(&mut self, sys: &System) -> usize {
        let mut steps = 0;
        loop {
            let pairs = self.active_pairs(sys);
            match pairs.first() {
                None => return steps,
                Some(&(a, b)) => {
                    self.apply(sys, a, b).expect("active pair must have a rule");
                    steps += 1;
                }
            }
        }
    }

    /// Export to the grammar substrate: agents become vertices, each wire an edge, and each
    /// vertex's rotation is its port order **principal first** — Gay's serialization of the
    /// geometric arrangement. The exported map answers to the same W1/W2 validity as any
    /// expression.
    pub fn to_map(&self, sys: &System) -> CombinatorialMap {
        // number the wires: each unordered port pair is one edge
        let mut edge_of: HashMap<Port, Dart> = HashMap::new();
        let mut next_edge = 0usize;
        for (&p, &q) in &self.partner {
            if p < q {
                edge_of.insert(p, 2 * next_edge);
                edge_of.insert(q, 2 * next_edge + 1);
                next_edge += 1;
            }
        }
        let mut rotations: Vec<(String, Vec<Dart>)> = Vec::new();
        for (id, sym) in self.agents.iter().enumerate() {
            let Some(sym) = *sym else { continue };
            let mut darts = Vec::new();
            for s in 0..=sys.arity(sym) {
                if let Some(&d) = edge_of.get(&(id, s)) {
                    darts.push(d);
                }
            }
            if !darts.is_empty() {
                rotations.push((format!("{}#{id}", sys.names[sym]), darts));
            }
        }
        CombinatorialMap::from_rotations(rotations, 2 * next_edge)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The unary-arithmetic system from the primary read (Gay 1991 §1): symbols Z, S, Add and
    /// the two rules for addition, plus explicit Erase and Dup — deletion and copying as agents.
    fn arithmetic() -> (System, SymbolId, SymbolId, SymbolId, SymbolId, SymbolId) {
        let mut sys = System::new();
        let z = sys.symbol("Z", 0);
        let s = sys.symbol("S", 1);
        let add = sys.symbol("Add", 2); // aux: [y, result]
        let erase = sys.symbol("Erase", 0);
        let dup = sys.symbol("Dup", 2);

        // Add(u,y,x), Z(u) -> x-y : the pass-through wire
        sys.rule(
            add,
            z,
            Rule {
                rhs_agents: vec![],
                links: vec![(RPort::Iface(0), RPort::Iface(1))],
            },
        )
        .unwrap();
        // Add(u,y,x), S(u,zp) -> S(x,w), Add(zp,y,w)
        sys.rule(
            add,
            s,
            Rule {
                rhs_agents: vec![s, add],
                links: vec![
                    (RPort::New(0, 0), RPort::Iface(1)), // S' principal — old result x
                    (RPort::New(0, 1), RPort::New(1, 2)), // S' aux — Add' result w
                    (RPort::New(1, 0), RPort::Iface(2)), // Add' principal — S's old aux zp
                    (RPort::New(1, 1), RPort::Iface(0)), // Add' y — old y
                ],
            },
        )
        .unwrap();
        // Erase, Z -> (nothing)
        sys.rule(
            erase,
            z,
            Rule {
                rhs_agents: vec![],
                links: vec![],
            },
        )
        .unwrap();
        // Erase, S(zp) -> Erase'(zp)
        sys.rule(
            erase,
            s,
            Rule {
                rhs_agents: vec![erase],
                links: vec![(RPort::New(0, 0), RPort::Iface(0))],
            },
        )
        .unwrap();
        // Dup(a,b), Z -> Z,Z
        sys.rule(
            dup,
            z,
            Rule {
                rhs_agents: vec![z, z],
                links: vec![
                    (RPort::New(0, 0), RPort::Iface(0)),
                    (RPort::New(1, 0), RPort::Iface(1)),
                ],
            },
        )
        .unwrap();
        // Dup(a,b), S(zp) -> S(a,w1), S(b,w2), Dup(zp,w1,w2)
        sys.rule(
            dup,
            s,
            Rule {
                rhs_agents: vec![s, s, dup],
                links: vec![
                    (RPort::New(0, 0), RPort::Iface(0)),
                    (RPort::New(1, 0), RPort::Iface(1)),
                    (RPort::New(0, 1), RPort::New(2, 1)),
                    (RPort::New(1, 1), RPort::New(2, 2)),
                    (RPort::New(2, 0), RPort::Iface(2)),
                ],
            },
        )
        .unwrap();
        (sys, z, s, add, erase, dup)
    }

    /// Build the unary numeral n; returns the port to wire (the outermost principal).
    fn num(net: &mut Net, z: SymbolId, s: SymbolId, n: usize) -> Port {
        let zid = net.add(z);
        let mut out: Port = (zid, 0);
        for _ in 0..n {
            let sid = net.add(s);
            net.wire((sid, 1), out);
            out = (sid, 0);
        }
        out
    }

    /// Read a unary numeral starting from a FREE handle.
    fn read_num(net: &Net, sys: &System, free_agent: usize, s: SymbolId, z: SymbolId) -> usize {
        let mut count = 0;
        let mut p = net.partner[&(free_agent, 0)];
        loop {
            let sym = net.symbol_of(p.0).expect("live agent");
            if sym == s {
                count += 1;
                p = net.partner[&(p.0, 1)];
            } else {
                assert_eq!(sym, z, "numeral must end in Z");
                assert_eq!(sys.arity(z), 0);
                return count;
            }
        }
    }

    #[test]
    fn two_plus_two_reduces_to_four() {
        let (sys, z, s, add, ..) = arithmetic();
        let mut net = Net::new();
        let a = num(&mut net, z, s, 2);
        let b = num(&mut net, z, s, 2);
        let adder = net.add(add);
        let result = net.free_port();
        net.wire((adder, 0), a); // principal meets the first numeral: the active pair
        net.wire((adder, 1), b); // y — passive
        net.wire((adder, 2), (result, 0));
        let steps = net.normalize(&sys);
        assert_eq!(steps, 3, "S, S, then Z");
        assert_eq!(read_num(&net, &sys, result, s, z), 4);
    }

    #[test]
    fn deletion_is_an_agent_not_an_ambient_right() {
        let (sys, z, s, _, erase, _) = arithmetic();
        let mut net = Net::new();
        let n = num(&mut net, z, s, 3);
        let e = net.add(erase);
        net.wire((e, 0), n);
        net.normalize(&sys);
        assert_eq!(
            net.live_agents(),
            0,
            "the numeral is consumed step by step, explicitly"
        );
    }

    #[test]
    fn copying_is_an_agent_not_an_ambient_right() {
        let (sys, z, s, _, _, dup) = arithmetic();
        let mut net = Net::new();
        let n = num(&mut net, z, s, 3);
        let d = net.add(dup);
        let out1 = net.free_port();
        let out2 = net.free_port();
        net.wire((d, 0), n);
        net.wire((d, 1), (out1, 0));
        net.wire((d, 2), (out2, 0));
        net.normalize(&sys);
        assert_eq!(read_num(&net, &sys, out1, s, z), 3);
        assert_eq!(read_num(&net, &sys, out2, s, z), 3);
    }

    #[test]
    fn linearity_is_enforced_at_registration() {
        let mut sys = System::new();
        let a = sys.symbol("A", 1);
        let b = sys.symbol("B", 1);
        // dropping interface 1: free delete — refused
        let drop = Rule {
            rhs_agents: vec![],
            links: vec![(RPort::Iface(0), RPort::Iface(0))],
        };
        let err = sys.rule(a, b, drop).unwrap_err();
        assert!(err.contains("no free copy") || err.contains("linearity"));
        // using interface 0 twice: free copy — refused
        let copy = Rule {
            rhs_agents: vec![],
            links: vec![
                (RPort::Iface(0), RPort::Iface(1)),
                (RPort::Iface(0), RPort::Iface(1)),
            ],
        };
        assert!(sys.rule(a, b, copy).is_err());
    }

    #[test]
    fn reduction_order_does_not_change_the_answer() {
        let (sys, z, s, add, ..) = arithmetic();
        let build = |net: &mut Net| {
            let mut results = Vec::new();
            for _ in 0..2 {
                let a = num(net, z, s, 2);
                let b = num(net, z, s, 1);
                let adder = net.add(add);
                let r = net.free_port();
                net.wire((adder, 0), a);
                net.wire((adder, 1), b);
                net.wire((adder, 2), (r, 0));
                results.push(r);
            }
            results
        };
        // left-to-right
        let mut n1 = Net::new();
        let r1 = build(&mut n1);
        n1.normalize(&sys);
        // right-to-left, one manual step at a time
        let mut n2 = Net::new();
        let r2 = build(&mut n2);
        loop {
            let pairs = n2.active_pairs(&sys);
            match pairs.last() {
                None => break,
                Some(&(a, b)) => n2.apply(&sys, a, b).unwrap(),
            }
        }
        for (x, y) in r1.iter().zip(r2.iter()) {
            assert_eq!(
                read_num(&n1, &sys, *x, s, z),
                read_num(&n2, &sys, *y, s, z),
                "strong confluence: same normal form either way"
            );
        }
    }

    #[test]
    fn every_net_state_is_a_valid_map_with_principal_first_rotations() {
        let (sys, z, s, add, ..) = arithmetic();
        let mut net = Net::new();
        let a = num(&mut net, z, s, 2);
        let b = num(&mut net, z, s, 2);
        let adder = net.add(add);
        let result = net.free_port();
        net.wire((adder, 0), a);
        net.wire((adder, 1), b);
        net.wire((adder, 2), (result, 0));

        // before reduction
        let m0 = net.to_map(&sys);
        assert!(
            m0.validate().is_ok(),
            "a program is a well-formed expression"
        );
        assert_eq!(m0.components(), 1);

        // after every step, still a valid expression
        while let Some(&(x, y)) = net.active_pairs(&sys).first() {
            net.apply(&sys, x, y).unwrap();
            let m = net.to_map(&sys);
            assert!(
                m.validate().is_ok(),
                "every intermediate state is a valid map"
            );
        }
        let mf = net.to_map(&sys);
        assert!(mf.validate().is_ok());
        // normal form of 2+2: Z, 4×S, one FREE handle -> 6 vertices, 5 edges, a path
        assert_eq!(mf.vertices().len(), 6);
        assert_eq!(mf.edge_count(), 5);
        assert_eq!(mf.genus(), Some(0));
    }
}
