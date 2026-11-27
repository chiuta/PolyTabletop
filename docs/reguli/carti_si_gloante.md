# Cărți și gloanțe (`carti_si_gloante`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play carti_si_gloante --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi vânători de recompense își dispută faima în orașul de la graniță. Fiecare erou (Pistolar / Pastor) are 22 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/2/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/2), «arta» (costă 2 focus, minte: lovitură de vrajă fără gardă: 3 PV), «odihna» (+2 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: un singur zar cu mărimea după pricepere (4 + 2×pricepere, maxim d12), care explodează la maxim (până la 3 ori), contra 3 + dificultate; cu 4 peste = excelent (o ratare la mustață dă „parțial”). După aruncare poți cheltui 1 noroc („noroc”): treapta crește cu 1. Regulă specială: o reușită excelentă dă o acțiune în plus (o dată pe tură). Mutarea «tinta» înseamnă o faptă: faima crește cu 1/2/3 după parțial/reușit/excelent; primul la 18 faimă câștigă; după 20 runde decide faima. Oboseală: de la runda 15 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, noroc.

Evenimentele MJ-ului (d6): 1 = avânt: cel din urmă +1 progres/PV; 2 = ajutor: cel din urmă +2 PV; 3 = capcană: amândoi −1 PV; 4 = șansă: cel din urmă +1 noroc; 5 = piedică: cel din față −1 progres/focus; 6 = avânt: cel din urmă +1 progres/PV.

Fișe: Pistolar (forță/iscusință/minte 1/1/4) contra Pastor (3/1/2); focus inițial 4, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Cards and Bullets.
