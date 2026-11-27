# Iluzia cenușie (`iluzia_cenusie`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play iluzia_cenusie --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi oameni scoși din iluzia cenușie se înfruntă între spaime. Fiecare erou (Dezabuzat / Medium) are 22 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/3/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 1 focus, minte: adversarul primește blestem (−1 la următoarea verificare, per ⌈2/2⌉) și 2/2 PV pierdute), «odihna» (+1 PV, +2 focus, −1 stres). Verificarea: pricepere zaruri d6 și contează cel mai mare (la pricepere 0: două zaruri, cel mai mic); 6 = reușit (două de 6 = excelent), 4–5 = parțial; dificultate 4+ scade zarul cu 1. La parțial poți plăti („plata”) 1 PV și rezultatul devine reușit. Regulă specială: cu stresul la jumătate din maxim sau mai mult, −1 la toate verificările. Câștigi când adversarul ajunge la 0 PV; după 14 runde decid PV. La 16 stres pierzi (ratările adaugă stres). Oboseală: de la runda 11 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tira, accept, plata.

Evenimentele MJ-ului (d6): 1 = capcană: amândoi −1 PV; 2 = izvor: amândoi +1 focus; 3 = primejdie: cel din față −1 PV; 4 = spaimă: amândoi +1 stres; 5 = ajutor: cel din urmă +2 PV; 6 = răgaz: amândoi −1 stres.

Fișe: Dezabuzat (forță/iscusință/minte 1/4/1) contra Medium (3/2/1); focus inițial 4, noroc inițial 0.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Grey Illusion.
