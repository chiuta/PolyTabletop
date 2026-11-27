# Insula Apelor Urcate (`insula_apelor_urcate`)

Categorie: Cooperativ. Echipa (Jucătorul 1) contra Jocului (Jucătorul 2, mutări forțate). Pornire: `polytabletop play insula_apelor_urcate --seed 1`. Sursa implementării: `polytabletop-games/src/c2_surv.rs` (specificația în `c2_specs.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cooperativă de supraviețuire: 4 oameni, resurse grâne, blocuri, funii; echipa (J1: Arhitecta și Marinarul, câte 2 acțiuni pe rând) adună resurse (cauta i: 0=Ferma (+2 grâne); 1=Cariera (+3 blocuri); 2=Pădurea (+2 funii); 3=Portul (+2 blocuri); fiecare loc o dată pe rundă), lucrează la proiectul „Digul” (lucru: 2 blocuri → +1; trebuie 9) și întreține moralul (ingrij: 1 grâne → +1 moral, maxim 5). J2 este Jocul: eveniment (forțat) — hrănire (grâne: jumătate din oameni, rotunjit în sus), apoi criza anunțată public (un anumit număr dintr-o resursă, altfel moral pierdut). Câștigă echipa (J1) terminând proiectul în 11 runde; Jocul (J2) câștigă dacă moralul sau oamenii se termină. Mutări: cauta i, lucru, ingrij, pas.

Informație ascunsă: doar ordinea pachetelor, care nu aparține vreunui jucător (nu există informație privată).
