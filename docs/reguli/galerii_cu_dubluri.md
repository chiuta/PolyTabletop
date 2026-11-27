# Galerii cu dubluri (`galerii_cu_dubluri`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play galerii_cu_dubluri --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Dublurile fac legea în galeriile adânci. Fiecare erou (Gladiator / Vrăjitor de mină) are 14 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/3/5 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 2 focus, minte: lovitură de vrajă fără gardă: 3 PV), «odihna» (+1 PV, +2 focus, −1 stres). Verificarea: pricepere + 1 zaruri d6 (cel puțin 2), se adună; pentru fiecare valoare apărută de cel puțin două ori se adaugă încă un zar d6 (maxim 2 în plus); prag 8 + 4×dificultate, cu 8 peste = excelent. Rezultatul se aplică imediat. Regulă specială: cine e lovit primește +1 focus. Câștigi când adversarul ajunge la 0 PV; după 14 runde decid PV. Oboseală: de la runda 11 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tira, accept.

Evenimentele MJ-ului (d6): 1 = primejdie: cel din față −1 PV; 2 = primejdie: cel din față −1 PV; 3 = capcană: amândoi −1 PV; 4 = capcană: amândoi −2 PV; 5 = liniște; 6 = ajutor: cel din urmă +2 PV.

Fișe: Gladiator (forță/iscusință/minte 4/1/1) contra Vrăjitor de mină (2/3/1); focus inițial 3, noroc inițial 0.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Galleries of Doubles.
