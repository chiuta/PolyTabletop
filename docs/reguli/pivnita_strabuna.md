# Pivnița străbună (`pivnita_strabuna`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play pivnita_strabuna --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi cercetași coboară într-o pivniță străbună, cu capcane și vlăguire. Fiecare erou (Cavaler / Cleric) are 13 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/2/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/2), «arta» (costă 3 focus, minte: te vindeci cu 5 PV), «odihna» (+2 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: d20 + 2×pricepere contra pragului 8 + 3×dificultate; 20 sau +7 peste prag = excelent, 1 = ratat. Rezultatul se aplică imediat. Regulă specială: o lovitură de 3+ PV adaugă 1 stres țintei. Mutarea «tinta» înseamnă a înainta pe drum (1/2/3 pași după parțial/reușit/excelent); primul la 7 câștigă; piedicile și primejdiile îl lovesc pe cel din față; după 20 runde decide drumul parcurs. Oboseală: de la runda 15 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept.

Evenimentele MJ-ului (d6): 1 = șansă: cel din urmă +1 noroc; 2 = ajutor: cel din urmă +2 PV; 3 = piedică: cel din față −1 progres/focus; 4 = izvor: amândoi +1 focus; 5 = capcană: amândoi −1 PV; 6 = avânt: cel din urmă +1 progres/PV.

Fișe: Cavaler (forță/iscusință/minte 1/1/4) contra Cleric (3/2/1); focus inițial 3, noroc inițial 0.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Ancestral Cellar.
