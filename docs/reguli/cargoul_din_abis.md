# Cargoul din abis (`cargoul_din_abis`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play cargoul_din_abis --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi oameni de pe cargoul din abis își dispută ultima gură de aer. Fiecare erou (Marinar / Biolog) are 26 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/3/5 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 3 focus, minte: adversarul e marcat: următoarea ta lovitură lovește cu +5), «odihna» (+1 PV, +2 focus, −1 stres). Verificarea: pricepere + 2 zaruri d6, fiecare 5+ e un succes; reușești cu 1 + dificultate/3 succese, cu 2 în plus e excelent, mai puține dar nenule = parțial. La ratat sau parțial poți forța („forteaza”): +1 stres și reiei aruncarea, o singură dată. Regulă specială: cu stresul la jumătate din maxim sau mai mult, −1 la toate verificările. Câștigi când adversarul ajunge la 0 PV; după 18 runde decid PV. La 16 stres pierzi (ratările adaugă stres). Oboseală: de la runda 13 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tira, accept, forteaza.

Evenimentele MJ-ului (d6): 1 = liniște; 2 = spaimă: amândoi +1 stres; 3 = șansă: cel din urmă +1 noroc; 4 = primejdie: cel din față −1 PV; 5 = capcană: amândoi −1 PV; 6 = primejdie: cel din față −2 PV.

Fișe: Marinar (forță/iscusință/minte 2/2/2) contra Biolog (2/3/1); focus inițial 3, noroc inițial 0.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Cargo from the Abyss.
