# Contrabandiștii cetii (`contrabandistii_cetii`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play contrabandistii_cetii --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi contrabandiști își dispută un jaf în ceață. Fiecare erou (Pilot / Spărgător) are 21 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 1/3/4 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 1/2/3), «arta» (costă 2 focus, minte: adversarul e marcat: următoarea ta lovitură lovește cu +3), «odihna» (+1 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: pricepere zaruri d6 și contează cel mai mare (la pricepere 0: două zaruri, cel mai mic); 6 = reușit (două de 6 = excelent), 4–5 = parțial; dificultate 4+ scade zarul cu 1. La parțial poți plăti („plata”) 1 PV și rezultatul devine reușit. Regulă specială: al doilea care joacă în rundă are +1 la verificări. Mutarea «tinta» înseamnă o faptă: faima crește cu 0/2/3 după parțial/reușit/excelent; primul la 18 faimă câștigă; după 18 runde decide faima. Oboseală: de la runda 13 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, plata.

Evenimentele MJ-ului (d6): 1 = piedică: cel din față −1 progres/focus; 2 = piedică: cel din față −2 progres/focus; 3 = ajutor: cel din urmă +2 PV; 4 = capcană: amândoi −1 PV; 5 = izvor: amândoi +1 focus; 6 = avânt: cel din urmă +1 progres/PV.

Fișe: Pilot (forță/iscusință/minte 4/1/1) contra Spărgător (1/4/1); focus inițial 4, noroc inițial 0.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Fog Smugglers.
