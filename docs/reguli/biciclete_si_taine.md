# Biciclete și taine (`biciclete_si_taine`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play biciclete_si_taine --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi prieteni pedalează spre taina de la marginea orașului. Fiecare erou (Biciclist viteaz / Biciclistă isteață) are 8 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/2/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 3 focus, minte: câștigi gardă 5+1 și vindeci 5/2 PV), «odihna» (+1 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: un singur zar cu mărimea după pricepere (4 + 2×pricepere, maxim d12), care explodează la maxim (până la 3 ori), contra 3 + dificultate; cu 4 peste = excelent. După aruncare poți cheltui 1 noroc („noroc”): treapta crește cu 1. Regulă specială: cu stresul la jumătate din maxim sau mai mult, −1 la toate verificările. Mutarea «tinta» înseamnă a înainta pe drum (0/2/3 pași după parțial/reușit/excelent); primul la 7 câștigă; piedicile și primejdiile îl lovesc pe cel din față; după 18 runde decide drumul parcurs. La 6 stres pierzi. Oboseală: de la runda 13 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, noroc.

Evenimentele MJ-ului (d6): 1 = capcană: amândoi −1 PV; 2 = răgaz: amândoi −1 stres; 3 = spaimă: amândoi +1 stres; 4 = piedică: cel din față −1 progres/focus; 5 = piedică: cel din față −2 progres/focus; 6 = ajutor: cel din urmă +2 PV.

Fișe: Biciclist viteaz (forță/iscusință/minte 1/2/3) contra Biciclistă isteață (2/2/2); focus inițial 2, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Bikes and Secrets.
