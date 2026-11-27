# Trepte de destin (`trepte_de_destin`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play trepte_de_destin --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi adepți urcă trepte de destin și se înfruntă. Fiecare erou (Adept al săbiei / Elementalist) are 17 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/2/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 2 focus, minte: reușita dă o acțiune în plus; parțialul dă +1 noroc), «odihna» (+1 PV, +2 focus, −1 stres). Verificarea: un singur zar cu mărimea după pricepere (4 + 2×pricepere, maxim d12), care explodează la maxim (până la 3 ori), contra 3 + dificultate; cu 4 peste = excelent. După aruncare poți cheltui 1 noroc („noroc”): treapta crește cu 1. Regulă specială: al doilea care joacă în rundă are +1 la verificări. Câștigi când adversarul ajunge la 0 PV; după 14 runde decid PV. Oboseală: de la runda 11 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tira, accept, noroc.

Evenimentele MJ-ului (d6): 1 = ajutor: cel din urmă +2 PV; 2 = capcană: amândoi −2 PV; 3 = izvor: amândoi +1 focus; 4 = ajutor: cel din urmă +1 PV; 5 = primejdie: cel din față −1 PV; 6 = capcană: amândoi −1 PV.

Fișe: Adept al săbiei (forță/iscusință/minte 1/4/1) contra Elementalist (3/2/1); focus inițial 2, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Steps of Destiny.
