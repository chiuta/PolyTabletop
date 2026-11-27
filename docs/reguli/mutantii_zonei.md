# Mutanții Zonei (`mutantii_zonei`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play mutantii_zonei --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi mutanți străbat Zona spre un refugiu legendar, plătind cu stres mutațiile. Fiecare erou (Hamalul / Fermecătorul) are 16 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/2/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/2), «arta» (costă 1 focus, minte: te vindeci cu 3 PV), «odihna» (+2 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: pricepere + 2 zaruri d6, fiecare 5+ e un succes; reușești cu 1 + dificultate/3 succese, cu 2 în plus e excelent, mai puține dar nenule = parțial. La ratat sau parțial poți forța („forteaza”): +1 stres și reiei aruncarea, o singură dată. Regulă specială: fiecare vrajă costă +1 stres. Mutarea «tinta» înseamnă a înainta pe drum (1/2/3 pași după parțial/reușit/excelent); primul la 11 câștigă; piedicile și primejdiile îl lovesc pe cel din față; după 20 runde decide drumul parcurs. La 13 stres pierzi. Oboseală: de la runda 15 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, forteaza.

Evenimentele MJ-ului (d6): 1 = avânt: cel din urmă +1 progres/PV; 2 = șansă: cel din urmă +1 noroc; 3 = spaimă: amândoi +1 stres; 4 = răgaz: amândoi −1 stres; 5 = liniște; 6 = primejdie: cel din față −1 PV.

Fișe: Hamalul (forță/iscusință/minte 2/1/3) contra Fermecătorul (1/1/4); focus inițial 4, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Mutants of the Zone.
