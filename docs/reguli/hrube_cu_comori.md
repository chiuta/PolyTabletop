# Hrube cu comori (`hrube_cu_comori`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play hrube_cu_comori --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi aventurieri coboară în hrube pline de comori, cursă spre cea mai adâncă încăpere. Fiecare erou (Hoț / Luptător) are 12 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/2/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 1 focus, minte: lovitură de vrajă fără gardă: 3 PV), «odihna» (+1 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: 2d6 + pricepere − dificultate: până la 6 ratat, 7–9 parțial, 10–12 reușit, 13+ excelent. La parțial poți plăti („plata”) 1 PV și rezultatul devine reușit. Fără regulă specială. Mutarea «tinta» înseamnă a înainta pe drum (0/2/3 pași după parțial/reușit/excelent); primul la 6 câștigă; piedicile și primejdiile îl lovesc pe cel din față; după 18 runde decide drumul parcurs. Oboseală: de la runda 13 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, plata.

Evenimentele MJ-ului (d6): 1 = liniște; 2 = avânt: cel din urmă +1 progres/PV; 3 = piedică: cel din față −1 progres/focus; 4 = piedică: cel din față −2 progres/focus; 5 = capcană: amândoi −1 PV; 6 = primejdie: cel din față −1 PV.

Fișe: Hoț (forță/iscusință/minte 4/1/1) contra Luptător (2/3/1); focus inițial 2, noroc inițial 0.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Treasure Burrows.
