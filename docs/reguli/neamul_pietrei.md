# Neamul Pietrei (`neamul_pietrei`)

Categorie: Cooperativ. Echipa (Jucătorul 1) contra Jocului (Jucătorul 2, mutări forțate). Pornire: `polytabletop play neamul_pietrei --seed 1`. Sursa implementării: `polytabletop-games/src/c2_surv.rs` (specificația în `c2_specs.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cooperativă de supraviețuire: 5 oameni, resurse carne, piatră, lemn; echipa (J1: Culegătoarea și Vânătorul, câte 2 acțiuni pe rând) adună resurse (cauta i: 0=Savana (+3 carne); 1=Cariera (+2 piatră); 2=Pădurea (+2 lemn); 3=Râul (+2 carne); fiecare loc o dată pe rundă), lucrează la proiectul „Tabăra” (lucru: 2 piatră → +1; trebuie 7) și întreține moralul (ingrij: 1 carne → +1 moral, maxim 7). J2 este Jocul: eveniment (forțat) — hrănire (carne: jumătate din oameni, rotunjit în sus), apoi criza anunțată public (un anumit număr dintr-o resursă, altfel moral pierdut). Câștigă echipa (J1) terminând proiectul în 10 runde; Jocul (J2) câștigă dacă moralul sau oamenii se termină. Mutări: cauta i, lucru, ingrij, pas.

Informație ascunsă: doar ordinea pachetelor, care nu aparține vreunui jucător (nu există informație privată).
