# Ordinul Luminii (`ordinul_luminii`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play ordinul_luminii --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Un cavaler al Ordinului Luminii se duelează cu un contrabandist pilot. Fiecare erou (Cavaler al luminii / Contrabandist) are 14 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/2/5 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/2), «arta» (costă 1 focus, minte: lovitură de vrajă fără gardă: 2 PV), «odihna» (+2 PV, +2 focus, −1 stres). Verificarea: pricepere + 2 zaruri d6, fiecare 6+ e un succes; reușești cu 1 + dificultate/3 succese, cu 2 în plus e excelent, mai puține dar nenule = parțial. După aruncare poți cheltui 1 noroc („noroc”): treapta crește cu 1. Regulă specială: o reușită excelentă dă o acțiune în plus (o dată pe tură). Câștigi când adversarul ajunge la 0 PV; după 20 runde decid PV. Oboseală: de la runda 15 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tira, accept, noroc.

Evenimentele MJ-ului (d6): 1 = capcană: amândoi −1 PV; 2 = izvor: amândoi +1 focus; 3 = primejdie: cel din față −2 PV; 4 = capcană: amândoi −2 PV; 5 = primejdie: cel din față −1 PV; 6 = ajutor: cel din urmă +2 PV.

Fișe: Cavaler al luminii (forță/iscusință/minte 1/1/4) contra Contrabandist (3/1/2); focus inițial 2, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Order of the Light.
