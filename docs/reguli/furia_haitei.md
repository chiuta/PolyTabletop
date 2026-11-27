# Furia haitei (`furia_haitei`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play furia_haitei --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: O haită în furie și un vânător se măsoară în luptă deschisă. Fiecare erou (Lup de război / Vânător de lupi) are 17 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/3/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 2 focus, minte: rănești cu 4 PV și te vindeci cu jumătate), «odihna» (+1 PV, +2 focus, −1 stres). Verificarea: pricepere + 2 zaruri d6, fiecare 5+ e un succes; reușești cu 1 + dificultate/3 succese, cu 2 în plus e excelent, mai puține dar nenule = parțial. La ratat sau parțial poți forța („forteaza”): +1 stres și reiei aruncarea, o singură dată. Regulă specială: cine e lovit primește +1 focus. Câștigi când adversarul ajunge la 0 PV; după 18 runde decid PV. La 9 stres pierzi. Oboseală: de la runda 13 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tira, accept, forteaza.

Evenimentele MJ-ului (d6): 1 = spaimă: amândoi +1 stres; 2 = primejdie: cel din față −1 PV; 3 = ajutor: cel din urmă +2 PV; 4 = izvor: amândoi +1 focus; 5 = răgaz: amândoi −1 stres; 6 = capcană: amândoi −1 PV.

Fișe: Lup de război (forță/iscusință/minte 1/2/3) contra Vânător de lupi (3/1/2); focus inițial 4, noroc inițial 0.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Fury of the Pack.
