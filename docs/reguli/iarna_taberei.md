# Iarna Taberei (`iarna_taberei`)

Categorie: Cooperativ. Echipa (Jucătorul 1) contra Jocului (Jucătorul 2, mutări forțate). Pornire: `polytabletop play iarna_taberei --seed 1`. Sursa implementării: `polytabletop-games/src/c2_surv.rs` (specificația în `c2_specs.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cooperativă de supraviețuire: 4 oameni, resurse hrană, materiale, combustibil; echipa (J1: Vânătorul și Medica, câte 2 acțiuni pe rând) adună resurse (cauta i: 0=Magazinul (+2 hrană); 1=Depozitul (+3 materiale); 2=Școala (+2 materiale); 3=Benzinăria (+3 combustibil); fiecare loc o dată pe rundă), lucrează la proiectul „Baricada” (lucru: 2 materiale → +1; trebuie 8) și întreține moralul (ingrij: 1 hrană → +1 moral, maxim 6). J2 este Jocul: eveniment (forțat) — hrănire (hrană: jumătate din oameni, rotunjit în sus), apoi criza anunțată public (un anumit număr dintr-o resursă, altfel moral pierdut). Câștigă echipa (J1) terminând proiectul în 10 runde; Jocul (J2) câștigă dacă moralul sau oamenii se termină. Mutări: cauta i, lucru, ingrij, pas.

Informație ascunsă: doar ordinea pachetelor, care nu aparține vreunui jucător (nu există informație privată).
