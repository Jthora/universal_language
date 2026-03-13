Consider the following:

---
A category C consists of a class Ob(C) of objects and for each pair (A,B) a set Hom_C(A,B) of morphisms, with composition ∘: Hom(B,C) × Hom(A,B) → Hom(A,C) satisfying associativity and identity: id_B ∘ f = f = f ∘ id_A. A functor F: C → D assigns to each object A ∈ C an object F(A) ∈ D and to each morphism f: A → B a morphism F(f): F(A) → F(B), preserving composition and identities.

A natural transformation η: F ⇒ G between functors F,G: C → D assigns to each A ∈ Ob(C) a morphism η_A: F(A) → G(A) such that for every f: A → B, G(f) ∘ η_A = η_B ∘ F(f). Natural transformations compose vertically: (ε ∘ η)_A = ε_A ∘ η_A.

An adjunction F ⊣ G between functors F: C → D and G: D → C consists of a natural isomorphism Hom_D(F(A),B) ≅ Hom_C(A,G(B)). The unit η: Id_C ⇒ GF and counit ε: FG ⇒ Id_D satisfy the triangle identities εF ∘ Fη = id_F and Gε ∘ ηG = id_G.

The categorical limit of a diagram D: J → C is an object lim D equipped with projections π_j: lim D → D(j) such that for any cone (X, f_j: X → D(j)), there exists a unique mediating morphism u: X → lim D with π_j ∘ u = f_j for all j ∈ J. Dually, the colimit is the universal cocone under D.

The Yoneda lemma states that for any functor F: C^{op} → Set and object A ∈ C, Nat(Hom(−,A), F) ≅ F(A), naturally in A and F. The Yoneda embedding y: C → [C^{op}, Set] sending A ↦ Hom(−,A) is fully faithful. A monad (T, η, μ) on C consists of an endofunctor T: C → C with unit η: Id → T and multiplication μ: T² → T satisfying μ ∘ Tμ = μ ∘ μT and μ ∘ Tη = id = μ ∘ ηT.
---

Now respond to the following:

Take the concept of 'democracy' and recursively decompose it into its fundamental components at five levels — from the concrete institutional level down to the most abstract philosophical primitive. At each level, show how the components compose to produce the level above.