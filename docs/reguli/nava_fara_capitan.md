# Nava fără Căpitan (`nava_fara_capitan`)

Categorie: Cooperativ. Echipa (Jucătorul 1) contra Jocului (Jucătorul 2, mutări forțate). Pornire: `polytabletop play nava_fara_capitan --seed 1`. Sursa implementării: `polytabletop-games/src/c2_surv.rs` (specificația în `c2_specs.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cooperativă de supraviețuire: 4 oameni, resurse aer, energie, piese; echipa (J1: Ofițerul și Inginera, câte 2 acțiuni pe rând) adună resurse (cauta i: 0=Punte (+2 aer); 1=Reactor (+3 energie); 2=Atelier (+3 piese); 3=Hangar (+2 energie); fiecare loc o dată pe rundă), lucrează la proiectul „Saltul” (lucru: 2 energie → +1; trebuie 8) și întreține moralul (ingrij: 1 aer → +1 moral, maxim 6). J2 este Jocul: eveniment (forțat) — hrănire (aer: jumătate din oameni, rotunjit în sus), apoi criza anunțată public (un anumit număr dintr-o resursă, altfel moral pierdut). Câștigă echipa (J1) terminând proiectul în 9 runde; Jocul (J2) câștigă dacă moralul sau oamenii se termină. Mutări: cauta i, lucru, ingrij, pas.

Informație ascunsă: doar ordinea pachetelor, care nu aparține vreunui jucător (nu există informație privată).
