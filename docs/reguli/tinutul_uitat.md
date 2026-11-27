# Ținutul uitat (`tinutul_uitat`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play tinutul_uitat --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi pelerini străbat un ținut uitat, hex cu hex. Fiecare erou (Pădurar / Pelerin) are 13 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/2/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 0/2/4), «arta» (costă 1 focus, minte: adversarul primește blestem (−1 la următoarea verificare, per ⌈3/2⌉) și 3/2 PV pierdute), «odihna» (+2 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: pricepere + 2 zaruri d6, fiecare 6+ e un succes; reușești cu 1 + dificultate/3 succese, cu 2 în plus e excelent, mai puține dar nenule = parțial. La ratat sau parțial poți forța („forteaza”): +1 stres și reiei aruncarea, o singură dată. Regulă specială: o lovitură de 3+ PV adaugă 1 stres țintei. Mutarea «tinta» înseamnă a înainta pe drum (1/2/2 pași după parțial/reușit/excelent); primul la 8 câștigă; piedicile și primejdiile îl lovesc pe cel din față; după 16 runde decide drumul parcurs. Oboseală: de la runda 12 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, forteaza.

Evenimentele MJ-ului (d6): 1 = izvor: amândoi +1 focus; 2 = avânt: cel din urmă +1 progres/PV; 3 = liniște; 4 = piedică: cel din față −2 progres/focus; 5 = capcană: amândoi −1 PV; 6 = avânt: cel din urmă +1 progres/PV.

Fișe: Pădurar (forță/iscusință/minte 1/3/2) contra Pelerin (2/3/1); focus inițial 2, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Forgotten Land.
