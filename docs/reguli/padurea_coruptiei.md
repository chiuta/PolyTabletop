# Pădurea corupției (`padurea_coruptiei`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play padurea_coruptiei --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi rătăcitori se bat în pădurea corupției, unde fiecare vrajă lasă urme. Fiecare erou (Vânător / Vrăjitoare) are 14 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 2/4/6 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/2), «arta» (costă 2 focus, minte: rănești cu 4 PV și te vindeci cu jumătate), «odihna» (+2 PV, +2 focus, −1 stres). Verificarea: se aruncă d100 și trebuie să fie sub țintă (50 + 12×(pricepere − dificultate), între 8 și 95); sub o cincime din țintă = excelent (o ratare la mustață dă „parțial”). La parțial poți plăti („plata”) 1 PV și rezultatul devine reușit. Regulă specială: fiecare vrajă costă +1 stres. Câștigi când adversarul ajunge la 0 PV; după 20 runde decid PV. La 7 stres pierzi. Oboseală: de la runda 15 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tira, accept, plata.

Evenimentele MJ-ului (d6): 1 = primejdie: cel din față −1 PV; 2 = primejdie: cel din față −1 PV; 3 = izvor: amândoi +1 focus; 4 = liniște; 5 = spaimă: amândoi +1 stres; 6 = ajutor: cel din urmă +1 PV.

Fișe: Vânător (forță/iscusință/minte 1/4/1) contra Vrăjitoare (4/1/1); focus inițial 3, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Forest of Corruption.
