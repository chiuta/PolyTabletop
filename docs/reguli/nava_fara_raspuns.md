# Nava fără răspuns (`nava_fara_raspuns`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play nava_fara_raspuns --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi membri ai echipajului fug spre capsula de salvare pe o navă fără răspuns. Fiecare erou (Inginer / Medic) are 10 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/2/5 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 0/2/4), «arta» (costă 2 focus, minte: adversarul pierde 4 focus și garda, plus ⌈4/2⌉ PV), «odihna» (+2 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: pricepere + 2 zaruri d6, fiecare 6+ e un succes; reușești cu 1 + dificultate/3 succese, cu 2 în plus e excelent, mai puține dar nenule = parțial. La ratat sau parțial poți forța („forteaza”): +1 stres și reiei aruncarea, o singură dată. Regulă specială: cu stresul la jumătate din maxim sau mai mult, −1 la toate verificările. Mutarea «tinta» înseamnă a înainta pe drum (1/2/2 pași după parțial/reușit/excelent); primul la 6 câștigă; piedicile și primejdiile îl lovesc pe cel din față; după 16 runde decide drumul parcurs. La 11 stres pierzi (ratările adaugă stres). Oboseală: de la runda 12 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, forteaza.

Evenimentele MJ-ului (d6): 1 = șansă: cel din urmă +1 noroc; 2 = primejdie: cel din față −1 PV; 3 = avânt: cel din urmă +1 progres/PV; 4 = spaimă: amândoi +1 stres; 5 = izvor: amândoi +1 focus; 6 = piedică: cel din față −1 progres/focus.

Fișe: Inginer (forță/iscusință/minte 1/1/4) contra Medic (2/1/3); focus inițial 2, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Silent Ship.
