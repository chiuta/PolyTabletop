# Ceremonia sabiei (`ceremonia_sabiei`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play ceremonia_sabiei --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi emisari își câștigă onoarea la o ceremonie a sabiei. Fiecare erou (Sabiaș / Curtean) are 10 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/2/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 3 focus, minte: câștigi gardă 5+1 și vindeci 5/2 PV), «odihna» (+1 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: pricepere + 2 zaruri d10 (10 explodează), păstrezi cele mai bune (pricepere la număr) și aduni, contra 8 + 6×dificultate; cu 8 peste = excelent. După aruncare poți cheltui 1 noroc („noroc”): treapta crește cu 1. Regulă specială: al doilea care joacă în rundă are +1 la verificări. Mutarea «tinta» înseamnă o faptă: faima crește cu 1/1/2 după parțial/reușit/excelent; primul la 7 faimă câștigă; după 14 runde decide faima. Oboseală: de la runda 11 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, noroc.

Evenimentele MJ-ului (d6): 1 = liniște; 2 = piedică: cel din față −2 progres/focus; 3 = avânt: cel din urmă +1 progres/PV; 4 = capcană: amândoi −1 PV; 5 = izvor: amândoi +1 focus; 6 = șansă: cel din urmă +1 noroc.

Fișe: Sabiaș (forță/iscusință/minte 1/4/1) contra Curtean (3/2/1); focus inițial 2, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Ceremony of the Sword.
