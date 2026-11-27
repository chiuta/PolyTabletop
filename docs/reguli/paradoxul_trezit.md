# Paradoxul trezit (`paradoxul_trezit`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play paradoxul_trezit --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Un mag trezit și un gardian se înfruntă; fiecare vrajă trezește paradoxul. Fiecare erou (Mag trezit / Gardian) are 22 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 2/4/6 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 0/2/4), «arta» (costă 2 focus, minte: adversarul pierde 3 focus și garda, plus ⌈3/2⌉ PV), «odihna» (+2 PV, +2 focus, −1 stres). Verificarea: pricepere zaruri d6 și contează cel mai mare (la pricepere 0: două zaruri, cel mai mic); 6 = reușit (două de 6 = excelent), 4–5 = parțial; dificultate 4+ scade zarul cu 1. La parțial poți plăti („plata”) 1 PV și rezultatul devine reușit. Regulă specială: fiecare vrajă costă +1 stres. Câștigi când adversarul ajunge la 0 PV; după 16 runde decid PV. La 14 stres pierzi. Oboseală: de la runda 12 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tira, accept, plata.

Evenimentele MJ-ului (d6): 1 = primejdie: cel din față −2 PV; 2 = liniște; 3 = ajutor: cel din urmă +1 PV; 4 = spaimă: amândoi +1 stres; 5 = șansă: cel din urmă +1 noroc; 6 = ajutor: cel din urmă +2 PV.

Fișe: Mag trezit (forță/iscusință/minte 1/1/4) contra Gardian (1/3/2); focus inițial 3, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Awakened Paradox.
