# Temnița dintr-un Pachet (`temnita_unui_pachet`)

Categorie: Cooperativ. Echipa (Jucătorul 1) contra Jocului (Jucătorul 2, mutări forțate). Pornire: `polytabletop play temnita_unui_pachet --seed 1`. Sursa implementării: `polytabletop-games/src/c2_quest.rs` (specificația în `c2_specs.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cooperativă de drum cu întâlniri: echipa (J1: Luptătorul și Hoața, câte 2 acțiuni) trece prin 5 întâlniri și o încercare finală (putere 11). Cărțile au trei feluri (Forță, Iscusință, Minte) și valori 1–4: joc k adaugă la progresul întâlnirii valoarea întreagă dacă felul se potrivește, altfel jumătate (rotunjit în sus). Când progresul ajunge la putere, întâlnirea cade și fiecare erou trage o carte în plus; avans (forțat) dezvăluie următoarea. J2 este Jocul: eveniment (forțat) — întâlnirea lovește un erou (8 viață). Câștigă echipa dacă învinge încercarea finală; Jocul câștigă dacă ambii eroi cad. Mutări: joc k, pas, avans.

Informație ascunsă: doar ordinea pachetelor, care nu aparține vreunui jucător (nu există informație privată).
