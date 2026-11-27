# Tabăra regelui (`tabara_regelui`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play tabara_regelui --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi cavaleri își dispută faima la tabăra regelui, între onoare și nebunie. Fiecare erou (Cavaler / Bardă) are 18 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/2/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 0/2/4), «arta» (costă 2 focus, minte: câștigi gardă 3+1 și vindeci 3/2 PV), «odihna» (+2 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: se aruncă 3d6 și suma trebuie să fie sub țintă (11 + 2×(pricepere − dificultate), între 5 și 16); cu 5 sub țintă = excelent (o ratare la mustață dă „parțial”). După aruncare poți cheltui 1 noroc („noroc”): treapta crește cu 1. Regulă specială: cu stresul la jumătate din maxim sau mai mult, −1 la toate verificările. Mutarea «tinta» înseamnă o faptă: faima crește cu 1/2/2 după parțial/reușit/excelent; primul la 16 faimă câștigă; după 16 runde decide faima. La 16 stres pierzi. Oboseală: de la runda 12 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, noroc.

Evenimentele MJ-ului (d6): 1 = spaimă: amândoi +1 stres; 2 = avânt: cel din urmă +1 progres/PV; 3 = spaimă: amândoi +1 stres; 4 = capcană: amândoi −1 PV; 5 = piedică: cel din față −1 progres/focus; 6 = primejdie: cel din față −1 PV.

Fișe: Cavaler (forță/iscusință/minte 1/2/3) contra Bardă (2/1/3); focus inițial 2, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The King's Camp.
