# Spadasinii mărilor (`spadasinii_marilor`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play spadasinii_marilor --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi spadasini ai mărilor își măsoară faima în abordaje. Fiecare erou (Spadasin / Căpitan) are 16 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/3/5 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 2 focus, minte: reușita dă o acțiune în plus; parțialul dă +1 noroc), «odihna» (+1 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: pricepere zaruri d6 și contează cel mai mare (la pricepere 0: două zaruri, cel mai mic); 6 = reușit (două de 6 = excelent), 4–5 = parțial; dificultate 4+ scade zarul cu 1. După aruncare poți cheltui 1 noroc („noroc”): treapta crește cu 1. Regulă specială: o reușită excelentă dă o acțiune în plus (o dată pe tură). Mutarea «tinta» înseamnă o faptă: faima crește cu 0/2/3 după parțial/reușit/excelent; primul la 14 faimă câștigă; după 18 runde decide faima. Oboseală: de la runda 13 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, noroc.

Evenimentele MJ-ului (d6): 1 = capcană: amândoi −1 PV; 2 = piedică: cel din față −1 progres/focus; 3 = piedică: cel din față −2 progres/focus; 4 = izvor: amândoi +1 focus; 5 = primejdie: cel din față −1 PV; 6 = ajutor: cel din urmă +2 PV.

Fișe: Spadasin (forță/iscusință/minte 1/3/2) contra Căpitan (3/1/2); focus inițial 3, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Swordsmen of the Seas.
