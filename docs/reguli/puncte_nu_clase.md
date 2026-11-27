# Puncte, nu clase (`puncte_nu_clase`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play puncte_nu_clase --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi eroi construiți din puncte se bat; reușita excelentă dă acțiune în plus. Fiecare erou (Gigant puternic / Ager) are 14 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 2/4/6 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/2), «arta» (costă 1 focus, minte: reușita dă o acțiune în plus; parțialul dă +1 noroc), «odihna» (+2 PV, +2 focus, −1 stres). Verificarea: se aruncă 3d6 și suma trebuie să fie sub țintă (11 + 2×(pricepere − dificultate), între 5 și 16); cu 5 sub țintă = excelent. Rezultatul se aplică imediat. Regulă specială: o reușită excelentă dă o acțiune în plus (o dată pe tură). Câștigi când adversarul ajunge la 0 PV; după 20 runde decid PV. Oboseală: de la runda 15 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tira, accept.

Evenimentele MJ-ului (d6): 1 = capcană: amândoi −2 PV; 2 = capcană: amândoi −1 PV; 3 = izvor: amândoi +1 focus; 4 = primejdie: cel din față −1 PV; 5 = ajutor: cel din urmă +2 PV; 6 = ajutor: cel din urmă +1 PV.

Fișe: Gigant puternic (forță/iscusință/minte 1/2/3) contra Ager (2/1/3); focus inițial 3, noroc inițial 0.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Points, Not Classes.
