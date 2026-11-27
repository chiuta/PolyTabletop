# Ceasul Alb-Negru (`ceasul_alb_negru`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play ceasul_alb_negru --seed 1`. Sursa implementării: `polytabletop-games/src/t4_tcg.rs` și `t4_tdata.rs`.

> Rezumat din `polytabletop list`: Duel original cu pachete fixe de 40 de cărți. Trei locuri în față, stocul crește cu 3 pe tură (maxim 8) și cu 1 la fiecare lovitură în jucător. O carte climax (cost 0) dă oricărui atacator +2 putere și +1 suflet în tura aceea. O lovitură în jucător scoate din pachet cărți-suflet în ceasul de daune; dacă iese o carte de anulare, toată lovitura se anulează. Șapte cărți în ceas fac un nivel; la nivelul 3 pierzi. Personajele se luptă după putere (cel mai slab cade, la egalitate cad ambele). Mutări: joaca i [tN], atac c t|p, gata. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Despre variantă

Joc original (joc de cărți colecționabile (pachete fixe)), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «weiß schwarz».

## Reguli

Duel original cu pachete fixe de 40 de cărți. Trei locuri în față, stocul crește cu 3 pe tură (maxim 8) și cu 1 la fiecare lovitură în jucător. O carte climax (cost 0) dă oricărui atacator +2 putere și +1 suflet în tura aceea. O lovitură în jucător scoate din pachet cărți-suflet în ceasul de daune; dacă iese o carte de anulare, toată lovitura se anulează. Șapte cărți în ceas fac un nivel; la nivelul 3 pierzi. Personajele se luptă după putere (cel mai slab cade, la egalitate cad ambele). Mutări: joaca i [tN], atac c t|p, gata. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Informație ascunsă și AI

Mâna fiecărui jucător este ascunsă (HOTSEAT_SAFE: `render` arată doar mâna privitorului), la fel cărțile zonei de viață/scuturilor; pachetul nu are ordine fixă, cărțile se trag la întâmplare cu un generator re-însămânțat la clonare, deci AI-ul nu vede cărțile viitoare. Evaluarea AI citește doar numărul cărților adversarului. Adâncime fixă, nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
