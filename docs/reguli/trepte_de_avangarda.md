# Treptele Avangardei (`trepte_de_avangarda`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play trepte_de_avangarda --seed 1`. Sursa implementării: `polytabletop-games/src/t4_tcg.rs` și `t4_tdata.rs`.

> Rezumat din `polytabletop list`: Duel original cu pachete fixe de 30 de cărți. Avangarda ta începe la nivelul 0; o dată pe tură urci (ride i) o carte cu un nivel mai sus peste ea. Chemi (joaca i) gratis unități de cel mult nivelul avangardei în 4 locuri din spate. Atacul avangardei descoperă mai întâi cartea de deasupra (un declanșator adaugă 2 putere și 1 daună), apoi apărătorul poate garda (garda i) cu cărți din mână, fiecare adăugând scutul ei; lovitura reușește dacă puterea e cel puțin puterea avangardei plus scuturile și adaugă cărți de daune (la 6 pierzi). Ceilalți atacatori retrag o unitate de putere egală sau mai mică. Mutări: ride i, joaca i, atac c|l t|p, garda i, nu, gata. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Despre variantă

Joc original (joc de cărți colecționabile (pachete fixe)), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «cardfight!! vanguard».

## Reguli

Duel original cu pachete fixe de 30 de cărți. Avangarda ta începe la nivelul 0; o dată pe tură urci (ride i) o carte cu un nivel mai sus peste ea. Chemi (joaca i) gratis unități de cel mult nivelul avangardei în 4 locuri din spate. Atacul avangardei descoperă mai întâi cartea de deasupra (un declanșator adaugă 2 putere și 1 daună), apoi apărătorul poate garda (garda i) cu cărți din mână, fiecare adăugând scutul ei; lovitura reușește dacă puterea e cel puțin puterea avangardei plus scuturile și adaugă cărți de daune (la 6 pierzi). Ceilalți atacatori retrag o unitate de putere egală sau mai mică. Mutări: ride i, joaca i, atac c|l t|p, garda i, nu, gata. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Informație ascunsă și AI

Mâna fiecărui jucător este ascunsă (HOTSEAT_SAFE: `render` arată doar mâna privitorului), la fel cărțile zonei de viață/scuturilor; pachetul nu are ordine fixă, cărțile se trag la întâmplare cu un generator re-însămânțat la clonare, deci AI-ul nu vede cărțile viitoare. Evaluarea AI citește doar numărul cărților adversarului. Adâncime fixă, nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
