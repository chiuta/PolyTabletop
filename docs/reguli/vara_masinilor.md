# Vara mașinilor (`vara_masinilor`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play vara_masinilor --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi copii descoperă taine într-o vară cu mașinării ciudate. Fiecare erou (Copil inventator / Copil curios) are 8 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/2/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 2 focus, minte: te vindeci cu 3 PV), «odihna» (+1 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: pricepere + 2 zaruri d6, fiecare 6+ e un succes; reușești cu 1 + dificultate/3 succese, cu 2 în plus e excelent, mai puține dar nenule = parțial. La ratat sau parțial poți forța („forteaza”): +1 stres și reiei aruncarea, o singură dată. Regulă specială: cu stresul la jumătate din maxim sau mai mult, −1 la toate verificările. Mutarea «tinta» înseamnă o faptă: faima crește cu 0/2/3 după parțial/reușit/excelent; primul la 6 faimă câștigă; după 18 runde decide faima. La 6 stres pierzi. Oboseală: de la runda 13 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, forteaza.

Evenimentele MJ-ului (d6): 1 = izvor: amândoi +1 focus; 2 = capcană: amândoi −1 PV; 3 = primejdie: cel din față −1 PV; 4 = piedică: cel din față −2 progres/focus; 5 = spaimă: amândoi +1 stres; 6 = piedică: cel din față −1 progres/focus.

Fișe: Copil inventator (forță/iscusință/minte 1/1/4) contra Copil curios (2/3/1); focus inițial 2, noroc inițial 0.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Summer of Machines.
