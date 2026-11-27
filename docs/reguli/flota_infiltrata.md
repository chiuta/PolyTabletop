# Flota Infiltrată (`flota_infiltrata`)

Categorie: Cooperativ. Echipa (Jucătorul 1) contra Jocului (Jucătorul 2, mutări forțate) — excepție: jocul în două faze are doi jucători reali. Pornire: `polytabletop play flota_infiltrata --seed 1`. Sursa implementării: `polytabletop-games/src/c2_betray.rs` (specificația în `c2_specs.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală în două faze, pentru doi jucători: faza 1 — explorare, pe rând (explora, apoi revela forțat) dintr-un pachet de 16 camere; fiecare cameră poate da un obiect sau un semn (și obiect); la 3 semne începe apariția și celălalt jucător decât cel care a dezvăluit semnul-limită devine trădător. Faza 2 — duel cu informație perfectă pe o linie 0..7: eroul (6+ viață) vrea să ajungă la capăt sau să doboare creatura (av, sprint cu obiect, lovesc, lovesc+, scut, pas); trădătorul mută creatura (mers, sare, izbeste dacă e lângă erou, blestem: paralizează eroul o tură). Câștigă cine își atinge scopul; după 70 de mutări câștigă trădătorul. Mutări: explora, revela, av, sprint, lovesc, lovesc+, scut, pas, mers, sare, izbeste, blestem.

Informație ascunsă: doar ordinea pachetelor, care nu aparține vreunui jucător (nu există informație privată).
