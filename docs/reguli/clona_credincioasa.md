# Clona credincioasă (`clona_credincioasa`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play clona_credincioasa --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Două clone loiale își toarnă una alteia vina; trădarea se adună. Fiecare erou (Clonă slujbașă / Clonă trădătoare) are 12 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 2/4/6 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 0/2/4), «arta» (costă 1 focus, minte: adversarul e marcat: următoarea ta lovitură lovește cu +2), «odihna» (+2 PV, +2 focus, −1 stres). Verificarea: pricepere + 1 zaruri d6 (cel puțin 2), se adună; pentru fiecare valoare apărută de cel puțin două ori se adaugă încă un zar d6 (maxim 2 în plus); prag 8 + 4×dificultate, cu 8 peste = excelent (o ratare la mustață dă „parțial”). Rezultatul se aplică imediat. Regulă specială: un rezultat ratat costă 1 PV. Câștigi când adversarul ajunge la 0 PV; după 16 runde decid PV. La 8 stres pierzi (ratările adaugă stres). Oboseală: de la runda 12 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tira, accept.

Evenimentele MJ-ului (d6): 1 = capcană: amândoi −2 PV; 2 = răgaz: amândoi −1 stres; 3 = capcană: amândoi −1 PV; 4 = izvor: amândoi +1 focus; 5 = primejdie: cel din față −1 PV; 6 = primejdie: cel din față −2 PV.

Fișe: Clonă slujbașă (forță/iscusință/minte 1/1/4) contra Clonă trădătoare (1/3/2); focus inițial 3, noroc inițial 0.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Loyal Clone.
