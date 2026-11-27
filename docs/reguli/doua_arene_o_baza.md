# Două Arene, o Bază (`doua_arene_o_baza`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play doua_arene_o_baza --seed 1`. Sursa implementării: `polytabletop-games/src/t4_tcg.rs` și `t4_tdata.rs`.

> Rezumat din `polytabletop list`: Duel original cu pachete fixe de 30 de cărți. Jucătorii alternează acțiuni singulare: joacă o carte, atacă cu o unitate pregătită sau dau pas (gata). Unitățile sunt de sol sau de aer: o unitate lovește doar gardienii și unitățile din arena ei, sau baza când nu stă niciun gardian din acea arenă. Unitățile proaspete nu pot ataca. Când amândoi dau pas la rând, runda se încheie: totul se pregătește, fiecare trage 2 și pune o carte din mână cu fața în jos ca resursă (pune i); cine a dat primul pas acționează primul. Bazele au 22 viață. Mutări: pune i, joaca i [tN], atac c t|p, gata. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Despre variantă

Joc original (joc de cărți colecționabile (pachete fixe)), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «star wars: unlimited».

## Reguli

Duel original cu pachete fixe de 30 de cărți. Jucătorii alternează acțiuni singulare: joacă o carte, atacă cu o unitate pregătită sau dau pas (gata). Unitățile sunt de sol sau de aer: o unitate lovește doar gardienii și unitățile din arena ei, sau baza când nu stă niciun gardian din acea arenă. Unitățile proaspete nu pot ataca. Când amândoi dau pas la rând, runda se încheie: totul se pregătește, fiecare trage 2 și pune o carte din mână cu fața în jos ca resursă (pune i); cine a dat primul pas acționează primul. Bazele au 22 viață. Mutări: pune i, joaca i [tN], atac c t|p, gata. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Informație ascunsă și AI

Mâna fiecărui jucător este ascunsă (HOTSEAT_SAFE: `render` arată doar mâna privitorului), la fel cărțile zonei de viață/scuturilor; pachetul nu are ordine fixă, cărțile se trag la întâmplare cu un generator re-însămânțat la clonare, deci AI-ul nu vede cărțile viitoare. Evaluarea AI citește doar numărul cărților adversarului. Adâncime fixă, nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
