# Jurământ de oțel (`juramant_de_otel`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play juramant_de_otel --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi purtători de jurământ străbat un ținut aspru spre ținta lor. Fiecare erou (Pelerin de oțel / Vânătoarea) are 13 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/2/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/2), «arta» (costă 2 focus, minte: câștigi gardă 3+1 și vindeci 3/2 PV), «odihna» (+2 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: un zar d6 + pricepere (minus dificultatea peste 1) contra a două zaruri d10: depășești ambele = reușit (excelent dacă cele două sunt egale), unul singur = parțial. Cu 2+ noroc și un rezultat sub reușit poți arde momentul („arde”): pierzi tot norocul, iar rezultatul devine reușit; reușitele aduc noroc. Regulă specială: o lovitură de 3+ PV adaugă 1 stres țintei. Mutarea «tinta» înseamnă a înainta pe drum (1/2/3 pași după parțial/reușit/excelent); primul la 8 câștigă; piedicile și primejdiile îl lovesc pe cel din față; după 20 runde decide drumul parcurs. Oboseală: de la runda 15 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, arde.

Evenimentele MJ-ului (d6): 1 = primejdie: cel din față −1 PV; 2 = ajutor: cel din urmă +2 PV; 3 = șansă: cel din urmă +1 noroc; 4 = piedică: cel din față −2 progres/focus; 5 = piedică: cel din față −1 progres/focus; 6 = liniște.

Fișe: Pelerin de oțel (forță/iscusință/minte 3/2/1) contra Vânătoarea (2/2/2); focus inițial 3, noroc inițial 2.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Steel Oath.
