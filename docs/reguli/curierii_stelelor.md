# Curierii stelelor (`curierii_stelelor`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play curierii_stelelor --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi curieri stelari se întrec spre portul de destinație prin salturi riscante. Fiecare erou (Pilot / Negustor) are 18 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/2/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 1 focus, minte: reușita dă o acțiune în plus; parțialul dă +1 noroc), «odihna» (+1 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: se aruncă 3d6 și suma trebuie să fie sub țintă (11 + 2×(pricepere − dificultate), între 5 și 16); cu 5 sub țintă = excelent. După aruncare poți cheltui 1 noroc („noroc”): treapta crește cu 1. Regulă specială: al doilea care joacă în rundă are +1 la verificări. Mutarea «tinta» înseamnă a înainta pe drum (1/1/2 pași după parțial/reușit/excelent); primul la 14 câștigă; piedicile și primejdiile îl lovesc pe cel din față; după 14 runde decide drumul parcurs. Oboseală: de la runda 11 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, noroc.

Evenimentele MJ-ului (d6): 1 = primejdie: cel din față −1 PV; 2 = avânt: cel din urmă +1 progres/PV; 3 = capcană: amândoi −1 PV; 4 = liniște; 5 = ajutor: cel din urmă +2 PV; 6 = avânt: cel din urmă +1 progres/PV.

Fișe: Pilot (forță/iscusință/minte 2/2/2) contra Negustor (3/1/2); focus inițial 2, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Star Couriers.
