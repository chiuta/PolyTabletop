# Echipajul exploratorilor (`echipajul_exploratorilor`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play echipajul_exploratorilor --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Un echipaj de exploratori caută o pistă prin necunoscut. Fiecare erou (Comandant / Inginer) are 14 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/2/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 1 focus, minte: te vindeci cu 3 PV), «odihna» (+1 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: d20 + 2×pricepere contra pragului 8 + 3×dificultate; 20 sau +7 peste prag = excelent, 1 = ratat (o ratare la mustață dă „parțial”). După aruncare poți cheltui 1 noroc („noroc”): treapta crește cu 1. Regulă specială: al doilea care joacă în rundă are +1 la verificări. Mutarea «tinta» înseamnă a înainta pe drum (1/1/2 pași după parțial/reușit/excelent); primul la 9 câștigă; piedicile și primejdiile îl lovesc pe cel din față; după 14 runde decide drumul parcurs. Oboseală: de la runda 11 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, noroc.

Evenimentele MJ-ului (d6): 1 = avânt: cel din urmă +1 progres/PV; 2 = primejdie: cel din față −1 PV; 3 = piedică: cel din față −1 progres/focus; 4 = izvor: amândoi +1 focus; 5 = avânt: cel din urmă +1 progres/PV; 6 = capcană: amândoi −1 PV.

Fișe: Comandant (forță/iscusință/minte 2/1/3) contra Inginer (1/1/4); focus inițial 2, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Explorers' Crew.
