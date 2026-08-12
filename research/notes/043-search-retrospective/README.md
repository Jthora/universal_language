# 043 — What we should have searched differently

**Type:** correction
**Opened:** 2026-08-12
**Status:** closed
**Thread:** audits the search practice across all 42 notes

---

## 1. What the record shows

| | Count |
|---|---|
| Web searches, roughly | **~40** |
| **Primary sources actually read** | **4** |
| Primary reads that **materially changed** a claim | **4 of 4** |
| Load-bearing negatives overturned by adversarial search | **5 of 9** |
| Preregistered predictions that failed *informatively* | **6** |

**Every primary read changed something.** Four for four. And roughly 90% of the evidence base came
from search summaries.

---

## 2. Four things that cost us, each earned by a specific failure

### 2a. The summary-to-primary ratio was backwards — **10:1 the wrong way**

`ICONIC-GROUNDING-ENABLES-CROSSPLAY` was registered at priority 0 on a search summary. The primary
read found **no modality ablation**, populations sharing **architecture and dataset**, and results
**dataset-dependent** (ZMI 0.898 on MNIST, **0.460** on CIFAR-10). The claim shrank to a fraction of
what was recorded.

`ZADROZNY-SCOPE` was built from a summary saying the constraint is *"syntax and lexical semantics."*
The SEP read found it is **three-part** — syntax, lexical items, **and the semantics of composites**.

> **A primary read has changed a claim every single time we have done one. We did it four times in
> forty searches.**

### 2b. We searched for the thing and never for its other names

The fixed-point object is called a **rotation system**, a **combinatorial map**, a **ribbon graph**,
a **fat graph**, and a **dessin d'enfant** — one object, five vocabularies, five literatures.

**We found dessins by accident at note `027`**, thirty notes after the object first appeared. One
query reaches one field's vocabulary.

### 2c. We never asked who had already failed

Note `037`'s historical survey — Leibniz, Wilkins, Solresol, Blissymbolics — was **the single most
informative crawl about UWS's actual prospects**, and it arrived at note 37 of 43.

**Solresol alone** — a universal language built *from music*, which failed by converting relations
into vocabulary — is worth more than most of what we found in the first thirty notes. It should have
been query three, not query three hundred.

### 2d. We used sources before checking what they are for

**AMR was cited as universality evidence twice**, the second time after the repo had already recorded
it as an English-only corpus. One query on a source's own stated scope, *before* using it, would have
prevented both.

---

## 3. What worked, and should not be lost in the correction

- **Preregistration (S2)** produced **six informative prediction failures.** Writing the expectation
  first is what converted "I was wrong" from embarrassment into data — `019`'s narrowing and `041`'s
  checked failure mode both depended on it.
- **Adversarial search (R1)** overturned **5 of 9** load-bearing negatives. Highest yield per query
  in the whole session.
- **Falsification-first ordering** (Roster A, `012`) was the most productive planned crawl, and it was
  designed to hurt.

**The correction is not "search more." It is "search in a different order and at a different depth."**

---

## 4. The rules this earns

### S8 — Read one primary before registering a priority-0 claim

**Four for four.** A summary is sufficient to *find* a source and never sufficient to *rest* on one.
If a claim is worth priority 0, it is worth one fetch.

### S9 — Synonym sweep before concluding novelty or absence

The same structure carries different names in different fields. **Before claiming something is
unprecedented, absent, or ours, search its name in at least three vocabularies.** Note `034`
concluded "no formalism uses rotation systems" without ever searching *ribbon graph* or *fat graph*.

### S10 — Failure-first survey, at the start

**Before building anything, find who tried and why they stopped.** Cheapest high-information query
available, and we ran it thirty-seven notes late. A field's graveyard is better documented than its
frontier.

### S11 — Scope-before-cite

**One query on a source's own limitations before using it as evidence** — what corpus, what
population, what the authors say it does not show. `check-caveats.rb` enforces the caveat *after* it
is known; this is how it becomes known.

---

## 5. What we never did at all

Recorded as gaps rather than rules, since their value is untested here:

- **Non-English literature.** Zero queries. UNL failed partly on English bias and we searched only
  in English.
- **Corpora and datasets rather than papers.** We looked for claims, never for data we could run
  something against.
- **Null results specifically.** *"X was tried and did not work"* is a different query from
  *"does X work"*, and publication bias means the first is harder to find and more valuable.
- **Practitioners rather than publications.** Nobody who has actually built a notation was consulted.

## 6. The honest summary

**The searching was reactive.** Almost every query answered a question already in front of us.
Note `010`'s rosters were the exception — planned crawls — and the one designed to falsify was the
most productive thing in the session.

> **We optimized for answering the current question and never for mapping the territory. That is why
> the most informative single crawl arrived at note 37.**
