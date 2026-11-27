# Scuturi și Mană (`scuturi_si_mana`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play scuturi_si_mana --seed 1`. Sursa implementării: `polytabletop-games/src/t4_tcg.rs` și `t4_tdata.rs`.

> Rezumat din `polytabletop list`: Duel original cu pachete fixe de 30 de cărți. O dată pe tură pui o carte din mână în zona de mană (pune i); o carte cere atâta mană neobosită cât costul ei, inclusiv una de culoarea ei. 5 scuturi ascunse apără fiecare jucător: o creatură atacatoare sparge unul (două dacă e dublă), cartea merge în mâna proprietarului, dar o carte cu declanșator se joacă imediat gratis. Gardienii se atacă primii; poți ataca creaturi inamice obosite; creaturile se bat după putere. Fără scuturi, o lovitură câștigă. Mutări: pune i, joaca i [tN], atac c t|p, gata. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Despre variantă

Joc original (joc de cărți colecționabile (pachete fixe)), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «duel masters».

## Reguli

Duel original cu pachete fixe de 30 de cărți. O dată pe tură pui o carte din mână în zona de mană (pune i); o carte cere atâta mană neobosită cât costul ei, inclusiv una de culoarea ei. 5 scuturi ascunse apără fiecare jucător: o creatură atacatoare sparge unul (două dacă e dublă), cartea merge în mâna proprietarului, dar o carte cu declanșator se joacă imediat gratis. Gardienii se atacă primii; poți ataca creaturi inamice obosite; creaturile se bat după putere. Fără scuturi, o lovitură câștigă. Mutări: pune i, joaca i [tN], atac c t|p, gata. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Informație ascunsă și AI

Mâna fiecărui jucător este ascunsă (HOTSEAT_SAFE: `render` arată doar mâna privitorului), la fel cărțile zonei de viață/scuturilor; pachetul nu are ordine fixă, cărțile se trag la întâmplare cu un generator re-însămânțat la clonare, deci AI-ul nu vede cărțile viitoare. Evaluarea AI citește doar numărul cărților adversarului. Adâncime fixă, nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
