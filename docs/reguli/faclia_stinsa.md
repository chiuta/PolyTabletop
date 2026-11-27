# Făclia stinsă (`faclia_stinsa`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play faclia_stinsa --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi căutători coboară cu făclia care scade, tot mai adânc. Fiecare erou (Hoț / Paznic) are 16 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/3/5 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 1 focus, minte: reușita dă o acțiune în plus; parțialul dă +1 noroc), «odihna» (+1 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: d20 + 2×pricepere contra pragului 8 + 3×dificultate; 20 sau +7 peste prag = excelent, 1 = ratat. După aruncare poți cheltui 1 noroc („noroc”): treapta crește cu 1. Regulă specială: al doilea care joacă în rundă are +1 la verificări. Mutarea «tinta» înseamnă a înainta pe drum (0/2/3 pași după parțial/reușit/excelent); primul la 16 câștigă; piedicile și primejdiile îl lovesc pe cel din față; după 18 runde decide drumul parcurs. La 18 stres pierzi (ratările adaugă stres). Oboseală: de la runda 13 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, noroc.

Evenimentele MJ-ului (d6): 1 = avânt: cel din urmă +1 progres/PV; 2 = spaimă: amândoi +1 stres; 3 = spaimă: amândoi +1 stres; 4 = izvor: amândoi +1 focus; 5 = piedică: cel din față −1 progres/focus; 6 = primejdie: cel din față −1 PV.

Fișe: Hoț (forță/iscusință/minte 4/1/1) contra Paznic (2/2/2); focus inițial 3, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Snuffed Torch.
