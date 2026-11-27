# Planul scăpat de sub control (`planul_scapat`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play planul_scapat --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi complotiști își văd planul scăpat de sub control. Fiecare erou (Planificator / Improvizator) are 18 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/3/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 1 focus, minte: adversarul primește blestem (−1 la următoarea verificare, per ⌈3/2⌉) și 3/2 PV pierdute), «odihna» (+1 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: patru zaruri cu +1/0/−1, plus pricepere − dificultate: 0 = parțial, 1–2 = reușit, 3+ = excelent, sub 0 = ratat (o ratare la mustață dă „parțial”). La parțial poți plăti („plata”) 1 PV și rezultatul devine reușit. Regulă specială: un rezultat ratat costă 1 PV. Mutarea «tinta» înseamnă o faptă: faima crește cu 0/2/3 după parțial/reușit/excelent; primul la 16 faimă câștigă; după 18 runde decide faima. La 16 stres pierzi (ratările adaugă stres). Oboseală: de la runda 13 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, plata.

Evenimentele MJ-ului (d6): 1 = capcană: amândoi −1 PV; 2 = șansă: cel din urmă +1 noroc; 3 = răgaz: amândoi −1 stres; 4 = ajutor: cel din urmă +2 PV; 5 = piedică: cel din față −2 progres/focus; 6 = liniște.

Fișe: Planificator (forță/iscusință/minte 4/1/1) contra Improvizator (1/1/4); focus inițial 4, noroc inițial 0.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Plan Gone Astray.
