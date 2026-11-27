# Tabele fără sfârșit (`tabele_fara_sfarsit`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play tabele_fara_sfarsit --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Un duel cu tabele lungi de răni, răni grele și efecte. Fiecare erou (Luptător / Vrăjitor) are 28 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 2/4/6 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 1 focus, minte: adversarul pierde 2 focus și garda, plus ⌈2/2⌉ PV), «odihna» (+1 PV, +2 focus, −1 stres). Verificarea: se aruncă d100 și trebuie să fie sub țintă (50 + 12×(pricepere − dificultate), între 8 și 95); sub o cincime din țintă = excelent (o ratare la mustață dă „parțial”). Rezultatul se aplică imediat. Regulă specială: o lovitură de 3+ PV adaugă 1 stres țintei. Câștigi când adversarul ajunge la 0 PV; după 18 runde decid PV. Oboseală: de la runda 13 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tira, accept.

Evenimentele MJ-ului (d6): 1 = capcană: amândoi −2 PV; 2 = liniște; 3 = ajutor: cel din urmă +1 PV; 4 = primejdie: cel din față −2 PV; 5 = izvor: amândoi +1 focus; 6 = capcană: amândoi −1 PV.

Fișe: Luptător (forță/iscusință/minte 1/3/2) contra Vrăjitor (2/2/2); focus inițial 2, noroc inițial 0.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Endless Tables.
