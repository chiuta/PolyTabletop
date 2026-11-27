# Pâlnia morților (`palnia_mortilor`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play palnia_mortilor --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi supraviețuitori fragili se bat într-o pâlnie a morților. Fiecare erou (Țăran viteaz / Cultist) are 28 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/3/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 2 focus, minte: adversarul primește blestem (−1 la următoarea verificare, per ⌈3/2⌉) și 3/2 PV pierdute), «odihna» (+1 PV, +2 focus, −1 stres). Verificarea: d20 + 2×pricepere contra pragului 8 + 3×dificultate; 20 sau +7 peste prag = excelent, 1 = ratat (o ratare la mustață dă „parțial”). După aruncare poți cheltui 1 noroc („noroc”): treapta crește cu 1. Regulă specială: un rezultat ratat costă 1 PV. Câștigi când adversarul ajunge la 0 PV; după 14 runde decid PV. Oboseală: de la runda 11 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tira, accept, noroc.

Evenimentele MJ-ului (d6): 1 = ajutor: cel din urmă +2 PV; 2 = capcană: amândoi −2 PV; 3 = capcană: amândoi −1 PV; 4 = primejdie: cel din față −2 PV; 5 = izvor: amândoi +1 focus; 6 = ajutor: cel din urmă +1 PV.

Fișe: Țăran viteaz (forță/iscusință/minte 1/1/4) contra Cultist (2/1/3); focus inițial 4, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Funnel of the Dead.
