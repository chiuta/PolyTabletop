# Ringul cu bilă (`ringul_cu_bila`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play ringul_cu_bila --seed 1`. Sursa implementării: `polytabletop-games/src/mn_engine.rs` (motor) și `polytabletop-games/src/mn_specs.rs` (tabelele acestui joc).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală de miniaturi pentru doi jucători pe o tablă pătrățită 7×9. Unități (hp/mișcare/rază/zaruri-forță/armură): a = Atacant 2/4/1/2/0; t = Tank 4/2/1/3/2 [KH]; r = Rapid 1/5/1/1/0 [L]. Pe tură poți activa 4 unități: fiecare se poate muta („mv a1>b2”) și apoi ataca („atq a1>b2”); încheii cu „fin”. Atacul are efect fix: forță minus armură (minim 1), fără zaruri. Scop: duci unități pe rândul de la capătul opus (ies din joc, câte 2 puncte); primul la 6 puncte sau cu mai multe după 14 runde câștigă. Flanc: +1 zar dacă un aliat e lângă țintă. Lovitura reușită împinge ținta (sau o rănește în plus dacă nu are loc). H = nu atacă după mișcare. L = zburător. K = împinge. Mutări: mv, atq, tira, fin.

Informație: publică (tabla, unitățile și punctele sunt vizibile; nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, după alegerea atacului; AI-ul, cu adâncime fixă, nu poate vedea rezultatele viitoare. Titlu în opt limbi: en The Ball Ring.
