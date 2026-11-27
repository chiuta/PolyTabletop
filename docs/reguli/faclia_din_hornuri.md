# Făclia din hornuri (drum: peșteră) (`faclia_din_hornuri`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play faclia_din_hornuri --seed 1`. Sursa implementării: `polytabletop-games/src/faclia_din_hornuri.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: drum cu întâlniri prin peșteră, 7 stații, zaruri d6 (reușită la ≥ 3). Alegi eroul (alege k): Pușcașul, Cercetașul, Predicatorul. Povestitorul automat scoate la fiecare stație o întâlnire (primejdie, teren sau om, cu pericol, câștig și rană). infrunt, furis sau vorbesc aruncă zarurile eroului; ai nevoie de atâtea reușite cât pericolul (±1 după fel); abordarea potrivită dă câștigul întreg, celelalte jumătate; eșecul costă PV. Variantele cu + cheltuiesc 1 torță pentru un zar în plus; odihnesc vindecă 2 PV. Reguli proprii: rănile sunt cu 1 mai mari, dar și câștigul cu 1; odihna vindecă 3 PV în loc de 2. Primul care termină drumul ia +3; la 0 PV pierzi 2 puncte și revii la jumătate de viață. Câștigă cine are mai multe puncte după 7 stații sau 18 ture. Mutări: alege k, infrunt, furis, vorbesc (cu +), odihnesc, capcana, obiect, tira.

Detalii: Eroi: Pușcașul (PV 13, luptă 3, furiș 1, vorbă 1, obiect «Armură cusută»); Cercetașul (PV 10, luptă 1, furiș 3, vorbă 1, obiect «Praf de ochi»); Predicatorul (PV 9, luptă 1, furiș 2, vorbă 3, obiect «Pungă de arginți»). Tabel de 9 întâlniri (3 primejdii, 3 terenuri, 3 oameni) cu pericol 1–3, câștig 2–5, rană 1–3.

Motor „drum cu întâlniri” (t2_drum). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Torch in the Chimneys (trek: cave).
