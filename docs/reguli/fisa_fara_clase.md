# Fișa fără clase (`fisa_fara_clase`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play fisa_fara_clase --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi aventurieri cu fișe fără clase, construite din puncte, se înfruntă într-un duel. Fiecare erou (Mercenar / Savant) are 14 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 2/4/6 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 0/2/4), «arta» (costă 1 focus, minte: adversarul e marcat: următoarea ta lovitură lovește cu +3), «odihna» (+2 PV, +2 focus, −1 stres). Verificarea: se aruncă 3d6 și suma trebuie să fie sub țintă (11 + 2×(pricepere − dificultate), între 5 și 16); cu 5 sub țintă = excelent (o ratare la mustață dă „parțial”). Rezultatul se aplică imediat. Regulă specială: o lovitură de 3+ PV adaugă 1 stres țintei. Câștigi când adversarul ajunge la 0 PV; după 16 runde decid PV. Oboseală: de la runda 12 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tira, accept.

Evenimentele MJ-ului (d6): 1 = primejdie: cel din față −1 PV; 2 = liniște; 3 = ajutor: cel din urmă +1 PV; 4 = șansă: cel din urmă +1 noroc; 5 = izvor: amândoi +1 focus; 6 = primejdie: cel din față −2 PV.

Fișe: Mercenar (forță/iscusință/minte 1/4/1) contra Savant (3/2/1); focus inițial 3, noroc inițial 0.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Classless Sheet.
