# Cavaler și magician (`cavaler_si_magician`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play cavaler_si_magician --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Un cavaler și un magician se înfruntă după reguli de onoare și vrăji. Fiecare erou (Cavaler / Magician) are 19 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 2/3/5 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 0/2/4), «arta» (costă 2 focus, minte: câștigi gardă 4+1 și vindeci 4/2 PV), «odihna» (+2 PV, +2 focus, −1 stres). Verificarea: se aruncă d100 și trebuie să fie sub țintă (50 + 12×(pricepere − dificultate), între 8 și 95); sub o cincime din țintă = excelent. După aruncare poți cheltui 1 noroc („noroc”): treapta crește cu 1. Regulă specială: al doilea care joacă în rundă are +1 la verificări. Câștigi când adversarul ajunge la 0 PV; după 16 runde decid PV. Oboseală: de la runda 12 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tira, accept, noroc.

Evenimentele MJ-ului (d6): 1 = primejdie: cel din față −2 PV; 2 = ajutor: cel din urmă +2 PV; 3 = șansă: cel din urmă +1 noroc; 4 = izvor: amândoi +1 focus; 5 = capcană: amândoi −1 PV; 6 = ajutor: cel din urmă +1 PV.

Fișe: Cavaler (forță/iscusință/minte 1/3/2) contra Magician (3/1/2); focus inițial 4, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Knight and Magician.
