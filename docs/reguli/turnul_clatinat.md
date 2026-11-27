# Turnul clătinat (`turnul_clatinat`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play turnul_clatinat --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Un turn de piese se clatină la fiecare încercare riscantă. Fiecare erou (Curajos / Prudent) are 12 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/2/5 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 0/2/4), «arta» (costă 1 focus, minte: adversarul e marcat: următoarea ta lovitură lovește cu +2), «odihna» (+2 PV, +2 focus, −1 stres). Verificarea: pricepere zaruri d6 și contează cel mai mare (la pricepere 0: două zaruri, cel mai mic); 6 = reușit (două de 6 = excelent), 4–5 = parțial; dificultate 4+ scade zarul cu 1. La ratat sau parțial poți forța („forteaza”): +1 stres și reiei aruncarea, o singură dată. Regulă specială: al doilea care joacă în rundă are +1 la verificări. Câștigi când adversarul ajunge la 0 PV; după 16 runde decid PV. La 7 stres pierzi. Oboseală: de la runda 12 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tira, accept, forteaza.

Evenimentele MJ-ului (d6): 1 = șansă: cel din urmă +1 noroc; 2 = izvor: amândoi +1 focus; 3 = liniște; 4 = primejdie: cel din față −2 PV; 5 = răgaz: amândoi −1 stres; 6 = ajutor: cel din urmă +1 PV.

Fișe: Curajos (forță/iscusință/minte 3/1/2) contra Prudent (3/2/1); focus inițial 2, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Swaying Tower.
