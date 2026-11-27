# Evoluția Circuitelor (`evolutia_circuitelor`)

Categorie: TCG. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play evolutia_circuitelor --seed 1`. Sursa implementării: `polytabletop-games/src/t4_tcg.rs` și `t4_tdata.rs`.

> Rezumat din `polytabletop list`: Duel original cu pachete fixe de 30 cărți și o măsură comună de memorie: începi cu 3, fiecare carte jucată sau evoluție cheltuiește costul, iar când memoria scade sub zero tura ta se încheie și adversarul pornește cu atât. Evoluezi (evo i j) o creatură de nivel N într-o carte de nivel N+1 de aceeași culoare cu 2 mai ieftin. Pază: 5 cărți ascunse; o lovitură în jucător răstoarnă una, iar o creatură mai puternică decât atacatorul îl distruge. Fără pază, o lovitură câștigă. Mutări: joaca i [tN], evo i j, atac c t|p, gata. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Despre variantă

Joc original (joc de cărți colecționabile (pachete fixe)), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «digimon card game».

## Reguli

Duel original cu pachete fixe de 30 cărți și o măsură comună de memorie: începi cu 3, fiecare carte jucată sau evoluție cheltuiește costul, iar când memoria scade sub zero tura ta se încheie și adversarul pornește cu atât. Evoluezi (evo i j) o creatură de nivel N într-o carte de nivel N+1 de aceeași culoare cu 2 mai ieftin. Pază: 5 cărți ascunse; o lovitură în jucător răstoarnă una, iar o creatură mai puternică decât atacatorul îl distruge. Fără pază, o lovitură câștigă. Mutări: joaca i [tN], evo i j, atac c t|p, gata. Informație ascunsă: vezi doar mâna ta (HOTSEAT_SAFE).

## Informație ascunsă și AI

Mâna fiecărui jucător este ascunsă (HOTSEAT_SAFE: `render` arată doar mâna privitorului), la fel cărțile zonei de viață/scuturilor; pachetul nu are ordine fixă, cărțile se trag la întâmplare cu un generator re-însămânțat la clonare, deci AI-ul nu vede cărțile viitoare. Evaluarea AI citește doar numărul cărților adversarului. Adâncime fixă, nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
