# Carierele cu Zaruri (`cariere_zaruri`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play cariere_zaruri --seed 1`. Sursa implementării: `polytabletop-games/src/cariere_zaruri.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial. Titlul în opt limbi se află în `polytabletop-games/data/i18n/titles_C5.tsv`.

> Variantă originală: fiecare are un sac de zaruri (6 novici și 2 străjeri). Tura începe cu trag (scoți cinci zaruri și le arunci); fețele de energie se adună, unitățile se așază pe teren (dep i, cel mult 3). Cu energia cumperi zaruri din rezerva publică (cumpar t); unitățile gata atacă (atac i j; j = 0 lovește direct dacă adversarul n-are unități) și câștigi glorie cât costă unitatea doborâtă (1 la lovitură directă). Primul la 12 glorie câștigă. Mutări: trag, dep i, cumpar t, atac i j, fin.
