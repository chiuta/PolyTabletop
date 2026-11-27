# Coborârea fără lumină (drum: mină) (`coborarea_fara_lumina`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play coborarea_fara_lumina --seed 1`. Sursa implementării: `polytabletop-games/src/coborarea_fara_lumina.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: drum cu întâlniri prin mină, 7 stații, zaruri d10 (reușită la ≥ 5). Alegi eroul (alege k): Spadasinul, Cercetașul, Tălmaciul. Povestitorul automat scoate la fiecare stație o întâlnire (primejdie, teren sau om, cu pericol, câștig și rană). infrunt, furis sau vorbesc aruncă zarurile eroului; ai nevoie de atâtea reușite cât pericolul (±1 după fel); abordarea potrivită dă câștigul întreg, celelalte jumătate; eșecul costă PV. Variantele cu + cheltuiesc 1 dor pentru un zar în plus; odihnesc vindecă 2 PV. Reguli proprii: rănile sunt cu 1 mai mari, dar și câștigul cu 1; «capcana» (1 dor): următoarea probă a adversarului cere o reușită în plus. Primul care termină drumul ia +3; la 0 PV pierzi 2 puncte și revii la jumătate de viață. Câștigă cine are mai multe puncte după 7 stații sau 18 ture. Mutări: alege k, infrunt, furis, vorbesc (cu +), odihnesc, capcana, obiect, tira.

Detalii: Eroi: Spadasinul (PV 13, luptă 3, furiș 1, vorbă 1, obiect «Topor greu»); Cercetașul (PV 10, luptă 1, furiș 3, vorbă 1, obiect «Pelerină de ceață»); Tălmaciul (PV 9, luptă 1, furiș 2, vorbă 3, obiect «Flaut fermecat»). Tabel de 9 întâlniri (3 primejdii, 3 terenuri, 3 oameni) cu pericol 1–3, câștig 2–5, rană 1–3.

Motor „drum cu întâlniri” (t2_drum). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en Descent Without Light (trek: mine).
