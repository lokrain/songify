# Songify — Discovery Summary for v1 (Music Theory & Analytics)

Date: 2025-11-11
Owner: Love
Scope: Real-time chord/key analysis and next-chord suggestions for producers (AU/VST3 + standalone)

---

## Readiness

**Assessment:** 93% ready to implement.
**Open items:** Mac self-hosted CI runner procurement date; exact virtual‑MIDI roadmap on Windows (post‑v1).

---

## Q1 — Primary customer & core job

**Choice:** A → then B (Songwriters/Producers primary; Students/Teachers secondary)
**Value:** 10/10 • **Complexity:** 18/21 • **Time:** 16 w (A core then B repackaging) • **Confidence:** 65%
**Pros:** Largest market; daily workflow; upsell path.
**Cons:** DAW integration burden; latency/UX bar.

---

## Q2 — Form factor & DAW coverage

**Choice:** B — VST3 + AU now, plus minimal standalone for auditioning
**Value:** 10/10 • **Complexity:** 18/21 • **Time:** +13 w • **Confidence:** 65%
**Pros:** Includes Logic; broad coverage; good demos.
**Cons:** Higher build/signing complexity.

---

## Q3 — Shipping stack (Rust + plugin shells)

**Choice:** A — Rust DSP/theory core via C‑ABI static lib + JUCE C++ shells
**Value:** 10/10 • **Complexity:** 18/21 • **Time:** 9 w • **Confidence:** 75%
**Pros:** Cross‑format stability; testable core; RT‑safe contract.
**Cons:** FFI glue; two build systems.

---

## Q4 — Input modality & latency

**Choice:** D — Full real‑time audio chord/key detection
**Value:** 9/10 • **Complexity:** 20/21 • **Time:** 9 w • **Confidence:** 5%
**Pros:** Maximum live value; works with guitars/voice.
**Cons:** Highest DSP complexity.
**Coupled with Q5 (engine choice) to ensure reliability.**

---

## Q5 — Detection engine strategy

**Choice:** E — Hybrid: deterministic DSP baseline (≤100 ms) + optional ML mode (hardware‑aware)
**Value:** 10/10 • **Complexity:** 18/21 • **Time:** 10–11 w • **Confidence:** 65%
**Pros:** Graceful degradation; fast ship; accuracy headroom.
**Cons:** Two pipelines to maintain.

---

## Q6 — v1 “Aha” workflow & output

**Choice:** A + thin slice of C — Live Chord HUD + Next‑Chord + MIDI Out + Drag‑MIDI + 4 style presets
**Value:** 10/10 • **Complexity:** 17/21 • **Time:** 6–7 w • **Confidence:** 70%
**Pros:** Strong in‑session value; immediate creative direction.
**Cons:** More UI polish than A alone.

---

## Q7 — Privacy, compute location, telemetry

**Choice:** B — On‑device by default; opt‑in cloud assists; telemetry opt‑in
**Value:** 10/10 • **Complexity:** 18/21 • **Time:** +5 w • **Confidence:** 65%
**Pros:** Trust for producers; unlocks heavier features later.
**Cons:** Auth/quotas/ops overhead for cloud tasks.

---

## Q8 — Monetization & licensing

**Choice:** C + D + 14‑day trial — Perpetual base + optional Pro subscription; add Rent‑to‑Own channel; EDU discount
**Value:** 10/10 • **Complexity:** 18/21 • **Time:** 7–9 w • **Confidence:** 70%
**Pros:** Offline base; clear upsell; channel reach.
**Cons:** Entitlements and pricing UX complexity.

---

## Q9 — Chord vocabulary, notation, enharmonics

**Choice:** Dual‑layer engine with **Advanced/Jazz view as default**, switchable to Simple/Nashville
**Value:** 10/10 • **Complexity:** 19/21 • **Time:** 3–4 w • **Confidence:** 70%
**Pros:** Theory‑truthful; user‑friendly mappings; future‑proof.
**Cons:** Mapping/spec/QA overhead.

---

## Q10 — Platforms, DAW QA matrix, code signing

**Choice:** A — macOS 12+ Universal (Intel+Apple Silicon) + Windows 10/11; Deep QA: Logic + Ableton; Basic: FL + Cubase; Tech preview: Reaper, Studio One, Bitwig
**Value:** 10/10 • **Complexity:** 18/21 • **Time:** +3–4 w • **Confidence:** 75%
**Prereqs:** Apple Dev ID, Windows EV cert.

---

## Q11 — Training data & evaluation

**Choice:** E — Hybrid datasets (public + synthetic + curated modern clips; partner data opportunistic)
**Value:** 10/10 • **Complexity:** 18/21 • **Time:** 5–7 w to eval‑ready • **Confidence:** 70%
**Targets:** WCSR ≥75% (std sets), Key ≥85%; see Q19/D for GA gates.

---

## Q12 — MIDI routing, plugin types, audition path

**Choice:** Option 2 — Dual plugin types (AU MIDI Effect + VST3 effect with MIDI out) **plus** desktop standalone; built‑in audition synth; **virtual‑MIDI macOS‑only at v1**
**Value:** 10/10 • **Complexity:** 19/21 • **Time:** +5 w • **Confidence:** 70%
**Pros:** Works in Logic and VST hosts; great onboarding; clear demo path.
**Cons:** Extra app to maintain; Win virtual‑MIDI deferred.

---

## Q13 — Architecture: language boundary and RT model

**Choice:** A — Rust staticlib via C ABI; JUCE shells; lock‑free SPSC; zero‑alloc audio thread
**Value:** 10/10 • **Complexity:** 18/21 • **Time:** 2–3 w to prototype • **Confidence:** 70%

---

## Q14 — Preset/state formats

**Choice:** E — Hybrid: rkyv for host state; FlatBuffers `.songifypreset` for user presets; JSON5 debug dump
**Value:** 10/10 • **Complexity:** 17/21 • **Time:** 2 w • **Confidence:** 50%

---

## Q15 — Distribution, installers, activation

**Choice:** E — Own installers + offline RSA license + delta auto‑updater; optional marketplace R2O; **Cert subchoice 1** (Apple Dev ID + EV Windows)
**Value:** 10/10 • **Complexity:** 18/21 • **Time:** +6–7 w • **Confidence:** 60%

---

## Q16 — In‑DAW UI layout

**Choice:** A + C + thin slice of D — Performance HUD default; Circle‑of‑Fifths Advanced tab; 2‑bar chord track
**Value:** 10/10 • **Complexity:** 19/21 • **Time:** +5–6 w • **Confidence:** 70%

---

## Q17 — Performance targets & hardware minimums

**Choice:** D — Tiered modes: Safe / Balanced (default) / Ultra
**Targets:** Safe ≤140 ms / ≤8% CPU; Balanced ≤95 ms / ≤10%; Ultra ≤75 ms / ≤12% (48 kHz, 128 buffer baseline)
**Value:** 10/10 • **Complexity:** 19/21 • **Time:** +3–4 w • **Confidence:** 40%

---

## Q18 — ML runtime & model footprint

**Choice:** F — Core ML on macOS + ONNX Runtime DirectML on Windows; streaming CRNN (<3 MB; 20 ms hop; int8)
**Value:** 10/10 • **Complexity:** 19/21 • **Time:** +5–6 w • **Confidence:** 65%

---

## Q19 — Scope freeze & GA quality gate

**Choice:** D — Quality‑first GA
**Gates:** WCSR ≥80%; Key ≥90%; CPU ≤8% in Balanced; crash‑free ≥99.7% (Logic/Ableton) and ≥99.5% (FL/Cubase/Reaper).
**Value:** 9/10 • **Complexity:** 20/21 • **Time:** 18–20 w • **Confidence:** 10%

---

## Q20 — Team & resourcing

**Choice:** C — 3 core devs (Rust DSP/RT, Data/ML, JUCE/RelEng) + 0.5 QA + 0.25–0.5 UX
**Value:** 10/10 • **Complexity:** 19/21 • **Time:** 16–18 w • **Confidence:** 65%

---

## Q21 — Data acquisition & annotation

**Choice:** F — Hybrid: heavy synthetic + curated modern stems (~600 clips) + small EDU cohort for QC
**Value:** 10/10 • **Complexity:** 18/21 • **Time:** 5–7 w • **Confidence:** 65%
**Budget:** ~€6k–€12k initial.

---

## Q22 — Org/compliance

**Choice:** A — Existing EU company; Apple Dev ID; EV cert; VAT OSS; GDPR disclosures
**Value:** 10/10 • **Complexity:** 16/21 • **Time:** 2–4 w admin • **Confidence:** 70%

---

## Q23 — Compliance status snapshot

**Choice:** A — Everything ready now (as per your acceptance)
**Value:** 10/10 • **Complexity:** 12/21 • **Time:** 0 • **Confidence:** 60%

---

## Q24 — CI/CD & validation

**Choice:** D — Hybrid CI: GitHub Actions for builds/sign/validators + one self‑hosted Mac DAW runner; **Interim:** Windows self‑hosted runner now; add Mac runner by private beta (Q24R A)
**Value:** 10/10 • **Complexity:** 18/21 • **Time:** +4 w now (+2 w when Mac arrives) • **Confidence:** 60%

---

## Q25 — GA pricing

**Choice:** B — Base €149 perpetual; Pro €9.99/mo; 14‑day trial; EDU −40%; R2O €7.99×24
**Value:** 10/10 • **Complexity:** 14/21 • **Time:** +1–2 w • **Confidence:** 45%

---

## Q26 — Private‑beta plan

**Choice:** E — Two‑wave beta (20 + 30) + EDU micro‑cohort (10)
**Value:** 10/10 • **Complexity:** 18/21 • **Time:** +6–7 w • **Confidence:** 65%
**Metrics:** NPS ≥60; purchase intent ≥70%; support SLA <24 h.

---

## Q27 — JUCE licensing

**Choice:** A — Purchase JUCE commercial license
**Value:** 10/10 • **Complexity:** 12/21 • **Time:** 0–1 w • **Confidence:** 70%

---

## Engineering guardrails (summary)

* Real‑time audio thread: no locks/alloc/syscalls; lock‑free rings; atomics for params.
* No network I/O on audio thread; cloud tasks off‑thread and opt‑in.
* Codesign/notarize AU/VST3; EV cert for Windows installers.
* Deterministic state: rkyv for host chunks; FlatBuffers for cross‑DAW presets.
* Performance gates enforced in CI with DAW smoke on Win now, Mac by beta.

---

## Immediate next steps

1. Procure Mac runner (mini or MacStadium) and set target date for private beta.
2. Start data plan F: acquire stems, set QC rubric, spin up synthetic renders.
3. Lock JUCE license and CI secrets; implement C‑ABI boundary skeleton.
4. Draft privacy/ToS text in repo; wire opt‑in telemetry toggles.
5. Build v1 UI scaffold: Performance HUD, Circle‑of‑Fifths tab, 2‑bar chord track.

---

*End of discovery summary.*
