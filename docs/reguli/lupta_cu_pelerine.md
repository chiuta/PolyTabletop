# Lupta cu pelerine (`lupta_cu_pelerine`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play lupta_cu_pelerine --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi eroi cu pelerine se înfruntă într-o ciocnire de puteri. Fiecare erou (Forțos / Telepat) are 20 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/3/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 3 focus, minte: lovitură de vrajă fără gardă: 5 PV), «odihna» (+1 PV, +2 focus, −1 stres). Verificarea: d20 + 2×pricepere contra pragului 8 + 3×dificultate; 20 sau +7 peste prag = excelent, 1 = ratat. La parțial poți plăti („plata”) 1 PV și rezultatul devine reușit. Regulă specială: o reușită excelentă dă o acțiune în plus (o dată pe tură). Câștigi când adversarul ajunge la 0 PV; după 14 runde decid PV. Oboseală: de la runda 11 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tira, accept, plata.

Evenimentele MJ-ului (d6): 1 = ajutor: cel din urmă +2 PV; 2 = primejdie: cel din față −1 PV; 3 = liniște; 4 = primejdie: cel din față −1 PV; 5 = izvor: amândoi +1 focus; 6 = primejdie: cel din față −2 PV.

Fișe: Forțos (forță/iscusință/minte 2/2/2) contra Telepat (1/3/2); focus inițial 4, noroc inițial 0.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Caped Clash.
