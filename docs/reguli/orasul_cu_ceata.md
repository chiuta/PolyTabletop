# Orașul cu Ceață (`orasul_cu_ceata`)

Categorie: Cooperativ. Echipa (Jucătorul 1) contra Jocului (Jucătorul 2, mutări forțate). Pornire: `polytabletop play orasul_cu_ceata --seed 1`. Sursa implementării: `polytabletop-games/src/c2_quest.rs` (specificația în `c2_specs.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cooperativă de drum cu întâlniri: echipa (J1: Anchetatorul și Bibliotecara, câte 2 acțiuni) trece prin 7 întâlniri și o încercare finală (putere 14). Cărțile au trei feluri (Curaj, Rațiune, Credință) și valori 1–4: joc k adaugă la progresul întâlnirii valoarea întreagă dacă felul se potrivește, altfel jumătate (rotunjit în sus). Când progresul ajunge la putere, întâlnirea cade și eroii se vindecă cu 1; avans (forțat) dezvăluie următoarea. J2 este Jocul: eveniment (forțat) — întâlnirea lovește un erou (7 viață), iar după 17 runde timpul expiră. Câștigă echipa dacă învinge încercarea finală; Jocul câștigă dacă ambii eroi cad. Mutări: joc k, pas, avans.

Informație ascunsă: doar ordinea pachetelor, care nu aparține vreunui jucător (nu există informație privată).
