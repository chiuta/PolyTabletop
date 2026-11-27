# Ruinele eonilor (`ruinele_eonilor`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului), MJ automat. Pornire: `polytabletop play ruinele_eonilor --seed 1`. Sursa implementării: `polytabletop-games/src/t1_rpg.rs` (motor parametrizat) cu fișa de valori din `polytabletop-games/src/t1_specs.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cu MJ automat, 2 jucători: Doi cercetători jefuiesc ruinele eonilor după faimă și relicve. Fiecare erou (Arhiv / Căutător) are 13 PV. La începutul fiecărei runde MJ-ul trage un eveniment cu d6; apoi fiecare pe rând alege o acțiune: «atac» (forță, dăunează 2/3/5 PV după parțial/reușit/excelent), «aparare» (iscusință, gardă 0/2/4), «arta» (costă 3 focus, minte: adversarul pierde 5 focus și garda, plus ⌈5/2⌉ PV), «odihna» (+2 PV, +2 focus, −1 stres), «tinta» (obiectivul). Verificarea: d20 + 2×pricepere contra pragului 8 + 3×dificultate; 20 sau +7 peste prag = excelent, 1 = ratat. După aruncare poți cheltui 1 noroc („noroc”): treapta crește cu 1. Regulă specială: fiecare vrajă costă +1 stres. Mutarea «tinta» înseamnă o faptă: faima crește cu 1/2/2 după parțial/reușit/excelent; primul la 9 faimă câștigă; după 16 runde decide faima. La 9 stres pierzi. Oboseală: de la runda 12 fiecare eveniment al MJ-ului costă 1 PV eroului din urmă (la egalitate, celui care joacă al doilea). Mutări: atac, aparare, arta, odihna, tinta, tira, accept, noroc.

Evenimentele MJ-ului (d6): 1 = ajutor: cel din urmă +2 PV; 2 = spaimă: amândoi +1 stres; 3 = avânt: cel din urmă +1 progres/PV; 4 = avânt: cel din urmă +1 progres/PV; 5 = izvor: amândoi +1 focus; 6 = răgaz: amândoi −1 stres.

Fișe: Arhiv (forță/iscusință/minte 4/1/1) contra Căutător (1/3/2); focus inițial 4, noroc inițial 1.

Informație: publică (zarurile și starea sunt vizibile; nu există mâini ascunse). Aruncările se fac doar în mutarea forțată `tira`, iar AI-ul (adâncime fixă 0) estimează rezultatul cu un generator local fix, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Ruins of Eons.
