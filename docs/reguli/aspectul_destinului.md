# Aspectul destinului (`aspectul_destinului`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play aspectul_destinului --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi eroi își construiesc faima prin aspecte, noroc și răsturnări de situație. Fiecare erou (Spadasin / Intrigant) are 13 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/3/5 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 2 focus, minte: câștigi gardă 4+1 și vindeci 4/2 PV), «odihna» (+1 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: patru zaruri cu +1/0/−1, plus pricepere − dificultate: 0 = parțial, 1–2 = reușit, 3+ = excelent, sub 0 = ratat. După aruncare poți cheltui 1 noroc („noroc”): treapta crește cu 1. Fără regulă specială. Mutarea «tinta» înseamnă o faptă: faima crește cu 1/1/2 după parțial/reușit/excelent; primul la 13 faimă câștigă; după 14 runde decide faima. Oboseală: de la runda 11 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, noroc.

Evenimentele MJ-ului (d6): 1 = avânt: cel din urmă +1 progres/PV; 2 = capcană: amândoi −1 PV; 3 = primejdie: cel din față −1 PV; 4 = liniște; 5 = piedică: cel din față −2 progres/focus; 6 = șansă: cel din urmă +1 noroc.

Fișe: Spadasin (forță/iscusință/minte 3/2/1) contra Intrigant (1/3/2); focus inițial 3, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Aspect of Destiny.
