

## Five Levels of Abstraction

**Level 1 — Physical Chemistry**
Cellulose and lignin in dry wood undergo exothermic pyrolysis and combustion when their temperature exceeds ~300°C. The reaction releases radiant and convective heat sufficient to raise adjacent dry fuel above ignition threshold, sustaining a chain of oxidation reactions across contiguous fuel.

**Level 2 — Thermodynamics & Transport**
A localized exothermic process in a combustible medium produces a thermal flux that exceeds the ignition energy of neighboring material. The medium's low moisture content minimizes the endothermic heat-sink of vaporization, so the energy surplus propagates spatially as a self-sustaining thermal front.

**Level 3 — Dynamical Systems / Feedback**
A self-reinforcing process propagates through a susceptible medium: each activated element outputs enough energy to activate its neighbors, creating a positive feedback loop. Propagation requires (a) a threshold-crossing activation, (b) sufficient coupling between neighbors, and (c) a medium that has not already been depleted or dampened. The system is a spatial chain reaction governed by local excitation and nearest-neighbor coupling.

**Level 4 — Formal / Network Propagation**
A signal $S$ at node $\xi$ transitions neighboring nodes $\xi'$ from quiescent to active when the coupling $g(\xi, \xi') \cdot |S(\xi)|^2$ exceeds a threshold $\theta(\xi')$. Each newly active node becomes a source, yielding a nonlinear wavefront:

$$\partial_\tau S(\xi) \;=\; D\,\nabla^2 S \;+\; f\bigl(|S|^2\bigr)S \;-\;\mu\, S$$

Propagation occurs when the local gain $f(|S|^2)$ exceeds dissipation $\mu$ across a connected, susceptible substrate.

**Level 5 — Abstract Principle**
*A local transformation that produces more of its own cause in neighboring loci will propagate through any medium where receptivity exceeds dissipation.*

---

The invariant across all five levels: **propagation requires that each activated element's output is sufficient to activate at least one neighbor** — the effective reproduction number $R > 1$. Whether the vocabulary is cellulose oxidation, thermal flux, feedback gain, coupled field amplitude, or cause-producing-cause, the core truth is identical: *self-sustaining spread is the consequence of local excess over threshold in a coupled, susceptible medium.*