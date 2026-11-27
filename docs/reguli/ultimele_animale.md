# Ultimele Animale (`ultimele_animale`)

Categorie: Cooperativ. Echipa (Jucătorul 1) contra Jocului (Jucătorul 2, mutări forțate). Pornire: `polytabletop play ultimele_animale --seed 1`. Sursa implementării: `polytabletop-games/src/c2_surv.rs` (specificația în `c2_specs.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cooperativă de supraviețuire: 3 oameni, resurse hrană, fonduri, teren; echipa (J1: Biologul și Ranger-ul, câte 2 acțiuni pe rând) adună resurse (cauta i: 0=Savana (+2 hrană); 1=Donații (+3 fonduri); 2=Rezervația (+2 teren); 3=Laboratorul (+2 fonduri); fiecare loc o dată pe rundă), lucrează la proiectul „Rezervația” (lucru: 2 fonduri → +1; trebuie 7) și întreține moralul (ingrij: 1 hrană → +1 moral, maxim 6). J2 este Jocul: eveniment (forțat) — hrănire (hrană: jumătate din oameni, rotunjit în sus), apoi criza anunțată public (un anumit număr dintr-o resursă, altfel moral pierdut). Câștigă echipa (J1) terminând proiectul în 9 runde; Jocul (J2) câștigă dacă moralul sau oamenii se termină. Mutări: cauta i, lucru, ingrij, pas.

Informație ascunsă: doar ordinea pachetelor, care nu aparține vreunui jucător (nu există informație privată).
