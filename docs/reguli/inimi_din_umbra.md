# Inimi din umbră (`inimi_din_umbra`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play inimi_din_umbra --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi adolescenți-monștri se țin în „sfori” de inimi. Fiecare erou (Vampir / Vârcolac) are 9 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/3/5 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 1 focus, minte: adversarul primește blestem (−1 la următoarea verificare, per ⌈2/2⌉) și 2/2 PV pierdute), «odihna» (+1 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: 2d6 + pricepere − dificultate: până la 6 ratat, 7–9 parțial, 10–12 reușit, 13+ excelent. La parțial poți plăti („plata”) 1 PV și rezultatul devine reușit. Regulă specială: cu stresul la jumătate din maxim sau mai mult, −1 la toate verificările. Mutarea «tinta» înseamnă o faptă: faima crește cu 1/1/2 după parțial/reușit/excelent; primul la 8 faimă câștigă; după 14 runde decide faima. La 8 stres pierzi. Oboseală: de la runda 11 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, plata.

Evenimentele MJ-ului (d6): 1 = primejdie: cel din față −1 PV; 2 = ajutor: cel din urmă +2 PV; 3 = capcană: amândoi −1 PV; 4 = avânt: cel din urmă +1 progres/PV; 5 = liniște; 6 = spaimă: amândoi +1 stres.

Fișe: Vampir (forță/iscusință/minte 3/2/1) contra Vârcolac (4/1/1); focus inițial 3, noroc inițial 0.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Hearts in Shadow.
