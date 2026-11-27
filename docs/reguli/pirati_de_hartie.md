# Pirații de Hârtie (`pirati_de_hartie`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pirati_de_hartie --seed 1`. Sursa implementării: `polytabletop-games/src/t4_tcg.rs` și `t4_tdata.rs`.

> Rezumat din `polytabletop list`: Duel original cu pachete fixe de 30 de cărți. Aurul crește cu 2 pe tură (maxim 10). Ai o zonă de viață de 5 cărți ascunse: o lovitură în căpitan trimite o carte în mână, iar la zonă goală pierzi. Căpitanul tău (putere 4) poate ataca. Poți ataca doar personaje inamice obosite, gardienii trebuie loviți primii, iar un căpitan e lovit doar de o putere cel puțin egală cu a lui. Mutări: joaca i [tN], atac c|l t|p, gata. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Despre variantă

Joc original (joc de cărți colecționabile (pachete fixe)), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «one piece card game».

## Reguli

Duel original cu pachete fixe de 30 de cărți. Aurul crește cu 2 pe tură (maxim 10). Ai o zonă de viață de 5 cărți ascunse: o lovitură în căpitan trimite o carte în mână, iar la zonă goală pierzi. Căpitanul tău (putere 4) poate ataca. Poți ataca doar personaje inamice obosite, gardienii trebuie loviți primii, iar un căpitan e lovit doar de o putere cel puțin egală cu a lui. Mutări: joaca i [tN], atac c|l t|p, gata. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Informație ascunsă și AI

Mâna fiecărui jucător este ascunsă (HOTSEAT_SAFE: `render` arată doar mâna privitorului), la fel cărțile zonei de viață/scuturilor; pachetul nu are ordine fixă, cărțile se trag la întâmplare cu un generator re-însămânțat la clonare, deci AI-ul nu vede cărțile viitoare. Evaluarea AI citește doar numărul cărților adversarului. Adâncime fixă, nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
