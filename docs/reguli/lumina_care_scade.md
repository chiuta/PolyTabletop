# Lumina care scade (`lumina_care_scade`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play lumina_care_scade --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Supraviețuitorii își spun adevărurile cât mai ține lumina. Fiecare erou (Supraviețuitor / Povestitoare) are 9 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/3/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 2 focus, minte: te vindeci cu 4 PV), «odihna» (+1 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: pricepere + 2 zaruri d6, fiecare 5+ e un succes; reușești cu 1 + dificultate/3 succese, cu 2 în plus e excelent, mai puține dar nenule = parțial. La ratat sau parțial poți forța („forteaza”): +1 stres și reiei aruncarea, o singură dată. Regulă specială: cu stresul la jumătate din maxim sau mai mult, −1 la toate verificările. Mutarea «tinta» înseamnă o faptă: faima crește cu 1/1/2 după parțial/reușit/excelent; primul la 8 faimă câștigă; după 14 runde decide faima. La 13 stres pierzi (ratările adaugă stres). Oboseală: de la runda 11 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, forteaza.

Evenimentele MJ-ului (d6): 1 = piedică: cel din față −1 progres/focus; 2 = șansă: cel din urmă +1 noroc; 3 = izvor: amândoi +1 focus; 4 = răgaz: amândoi −1 stres; 5 = primejdie: cel din față −1 PV; 6 = avânt: cel din urmă +1 progres/PV.

Fișe: Supraviețuitor (forță/iscusință/minte 1/2/3) contra Povestitoare (3/1/2); focus inițial 4, noroc inițial 0.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Waning Light.
