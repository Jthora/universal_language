

## Five Levels of Abstraction

**Core truth preserved:** *A local transformation releases energy that triggers the same transformation in adjacent material, producing self-sustaining spatial propagation.*

---

**Level 1 — Physical Chemistry**
Cellulose in dry wood undergoes exothermic oxidative pyrolysis when heated above ~300°C. The reaction releases radiant and convective energy that raises neighboring cellulose past the same threshold. The chain continues as long as fuel and oxygen are present.

**Level 2 — Physics (Reaction–Diffusion)**
An exothermic reaction in a spatially extended medium produces a self-propagating front: the energy released at each point exceeds the activation energy of adjacent points, so the reaction zone advances as a traveling wave through the fuel field.

**Level 3 — Dynamical Systems**
A bistable reaction-diffusion system with states {unburnt, burning}. Local nonlinear kinetics (ignition threshold) coupled with spatial diffusion of the activator (heat) produce a stable traveling-wave solution — a front that converts the metastable state into the stable one, sustained by positive feedback between local release and neighbor coupling.

**Level 4 — Network Cascade**
A threshold cascade on a coupled lattice: each node holds stored potential; when a node's input exceeds its threshold it fires, releasing energy into its neighbors, which may then fire in turn. Propagation requires only that (a) each firing event produces enough output to exceed at least one neighbor's threshold, and (b) the substrate has sufficient connectivity and stored potential. The fire is a percolation of activation across a susceptible graph.

**Level 5 — Recursive Fixed Point**
The process is a fixed point of its own generating functor:

$$\Phi \cong F(\Phi) : \text{colim}[F^0(\varnothing) \to F^1(\varnothing) \to \cdots]$$

A local transformation $F$ maps substrate state to (transformed state + coupling flux). The fire *is* the colimit of the iterated self-application of $F$ over a spatially indexed domain — each stage $F^{n+1}(\varnothing)$ is determined entirely by applying $F$ to $F^n(\varnothing)$ via the adjacency morphisms. Propagation is the natural transformation $\eta$ carrying the structure forward through the chain. The cascade stabilizes (or exhausts) when it reaches its least fixed point. Substrate, threshold, and coupling are parameters of $F$; the *principle* is that any $F$ that is locally energy-surplus and neighbor-coupled yields a self-similar propagating fixed-point structure — fire, epidemic, rumor, or phase transition alike.

---

At every level, the same invariant holds: **the output of the local process is sufficient to initiate the same process in a neighbor, so the structure reproduces itself across space by its own action.**