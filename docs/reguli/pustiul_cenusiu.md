# Pustiul cenușiu (`pustiul_cenusiu`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play pustiul_cenusiu --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Un vânător și o căpetenie se înfruntă într-un pustiu cenușiu. Fiecare erou (Vânător / Căpetenie) are 13 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 2/3/5 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 0/2/4), «arta» (costă 1 focus, minte: adversarul e marcat: următoarea ta lovitură lovește cu +2), «odihna» (+2 PV, +2 focus, −1 stres). Verificarea: 2d6 + pricepere − dificultate: până la 6 ratat, 7–9 parțial, 10–12 reușit, 13+ excelent. La parțial poți plăti („plata”) 1 PV și rezultatul devine reușit. Regulă specială: cine e lovit primește +1 focus. Câștigi când adversarul ajunge la 0 PV; după 16 runde decid PV. Oboseală: de la runda 12 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tira, accept, plata.

Evenimentele MJ-ului (d6): 1 = ajutor: cel din urmă +1 PV; 2 = capcană: amândoi −1 PV; 3 = primejdie: cel din față −2 PV; 4 = ajutor: cel din urmă +2 PV; 5 = șansă: cel din urmă +1 noroc; 6 = capcană: amândoi −2 PV.

Fișe: Vânător (forță/iscusință/minte 2/1/3) contra Căpetenie (3/1/2); focus inițial 4, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Ashen Waste.
