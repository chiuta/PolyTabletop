# Jurământ și obstacol (`jurament_si_obstacol`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play jurament_si_obstacol --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi eroi își țin jurămintele și înfruntă obstacolele. Fiecare erou (Jurătorul / Vagabondul) are 20 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/2/5 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/2), «arta» (costă 3 focus, minte: câștigi gardă 5+1 și vindeci 5/2 PV), «odihna» (+2 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: pricepere + 2 zaruri d10 (10 explodează), păstrezi cele mai bune (pricepere la număr) și aduni, contra 8 + 6×dificultate; cu 8 peste = excelent (o ratare la mustață dă „parțial”). După aruncare poți cheltui 1 noroc („noroc”): treapta crește cu 1. Regulă specială: o lovitură de 3+ PV adaugă 1 stres țintei. Mutarea «tinta» înseamnă o faptă: faima crește cu 1/2/3 după parțial/reușit/excelent; primul la 16 faimă câștigă; după 20 runde decide faima. Oboseală: de la runda 15 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, noroc.

Evenimentele MJ-ului (d6): 1 = avânt: cel din urmă +1 progres/PV; 2 = ajutor: cel din urmă +2 PV; 3 = avânt: cel din urmă +1 progres/PV; 4 = piedică: cel din față −2 progres/focus; 5 = izvor: amândoi +1 focus; 6 = piedică: cel din față −1 progres/focus.

Fișe: Jurătorul (forță/iscusință/minte 4/1/1) contra Vagabondul (2/3/1); focus inițial 2, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Oath and Obstacle.
