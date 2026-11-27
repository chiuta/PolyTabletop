# Vizuina cu comori (drum: peșteră) (`vizuina_cu_comori`)

Categorie: Rol (RPG). Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play vizuina_cu_comori --seed 1`. Sursa implementării: `polytabletop-games/src/vizuina_cu_comori.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: drum cu întâlniri prin peșteră, 7 stații, zaruri d6 (reușită la ≥ 3). Alegi eroul (alege k): Străjerul, Pândarul, Vraciul. Povestitorul automat scoate la fiecare stație o întâlnire (primejdie, teren sau om, cu pericol, câștig și rană). infrunt, furis sau vorbesc aruncă zarurile eroului; ai nevoie de atâtea reușite cât pericolul (±1 după fel); abordarea potrivită dă câștigul întreg, celelalte jumătate; eșecul costă PV. Variantele cu + cheltuiesc 1 mană pentru un zar în plus; odihnesc vindecă 2 PV. Reguli proprii: «obiect»: o dată pe joc, reușită sigură cu câștig întreg; odihna vindecă 3 PV în loc de 2. Primul care termină drumul ia +3; la 0 PV pierzi 2 puncte și revii la jumătate de viață. Câștigă cine are mai multe puncte după 7 stații sau 18 ture. Mutări: alege k, infrunt, furis, vorbesc (cu +), odihnesc, capcana, obiect, tira.

Detalii: Eroi: Străjerul (PV 13, luptă 3, furiș 1, vorbă 1, obiect «Scut de lemn»); Pândarul (PV 10, luptă 1, furiș 3, vorbă 1, obiect «Cheie-scheletică»); Vraciul (PV 9, luptă 1, furiș 2, vorbă 3, obiect «Scrisoare de liberă trecere»). Tabel de 9 întâlniri (3 primejdii, 3 terenuri, 3 oameni) cu pericol 1–3, câștig 2–5, rană 1–3.

Motor „drum cu întâlniri” (t2_drum). Informație: publică (nu există mâini ascunse). Zarurile se aruncă doar în mutarea forțată `tira`, pe care AI-ul (cu adâncime fixă) nu o execută în căutare, deci nu poate vedea aruncările viitoare. Titlu în opt limbi: en The Treasure Burrow (trek: cave).
