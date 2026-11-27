# Energia Luptătorilor (`energia_luptatorilor`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play energia_luptatorilor --seed 1`. Sursa implementării: `polytabletop-games/src/t4_tcg.rs` și `t4_tdata.rs`.

> Rezumat din `polytabletop list`: Duel original cu pachete fixe de 30 cărți. Energia vine din mână: o dată pe tură încarci o carte (pune i) în zona de energie; toată energia e gata la începutul turei. Căpitanul tău (putere 3, +2 trezit când viața e la jumătate sau mai puțin) începe cu 8 viață. Atacă doar personaje inamice obosite sau căpitanul; întâi gardienii; un luptător dublu lovește de 2 ori. Câștigi ducând căpitanul adversarului la 0. Mutări: pune i, joaca i [tN], atac c|l t|p, gata. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Despre variantă

Joc original (joc de cărți colecționabile (pachete fixe)), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «dragon ball super card game».

## Reguli

Duel original cu pachete fixe de 30 cărți. Energia vine din mână: o dată pe tură încarci o carte (pune i) în zona de energie; toată energia e gata la începutul turei. Căpitanul tău (putere 3, +2 trezit când viața e la jumătate sau mai puțin) începe cu 8 viață. Atacă doar personaje inamice obosite sau căpitanul; întâi gardienii; un luptător dublu lovește de 2 ori. Câștigi ducând căpitanul adversarului la 0. Mutări: pune i, joaca i [tN], atac c|l t|p, gata. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Informație ascunsă și AI

Mâna fiecărui jucător este ascunsă (HOTSEAT_SAFE: `render` arată doar mâna privitorului), la fel cărțile zonei de viață/scuturilor; pachetul nu are ordine fixă, cărțile se trag la întâmplare cu un generator re-însămânțat la clonare, deci AI-ul nu vede cărțile viitoare. Evaluarea AI citește doar numărul cărților adversarului. Adâncime fixă, nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
