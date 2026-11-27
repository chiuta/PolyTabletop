# Foc și Pulbere (`foc_si_pulbere`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play foc_si_pulbere --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 11×8 pătrată (4 direcții) cu dărâmături (adăpost) și câțiva pereți. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Brigadă cu pușcă 4/3/4/3/4; Tunuri de câmp 3/1/6/4/4; Cavalerie de recunoaștere 3/6/1/2/4; General de armată 3/5/1/3/4. J1: 3× Brigadă cu pușcă, Tunuri de câmp, Cavalerie de recunoaștere, General de armată. J2: 3× Brigadă cu pușcă, Tunuri de câmp, Cavalerie de recunoaștere, General de armată. Atac: 2d6 plus zarurile unității trebuie să atingă 5 plus ținta; la o diferență de 4 se adaugă o pagubă. Reguli speciale: moral (șocul slăbește unitățile; r reface moralul); imobilizare (focul la distanță încetinește ținta). Unități speciale: lideri. Câștigi distrugând toate unitățile inamice. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), r uN, end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «fire and fury».

## Reguli

Variantă originală pentru doi pe o tablă 11×8 pătrată (4 direcții) cu dărâmături (adăpost) și câțiva pereți. La fiecare tură ai 3 puncte de acțiune; fiecare mutare, atac sau acțiune specială costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Brigadă cu pușcă 4/3/4/3/4; Tunuri de câmp 3/1/6/4/4; Cavalerie de recunoaștere 3/6/1/2/4; General de armată 3/5/1/3/4. J1: 3× Brigadă cu pușcă, Tunuri de câmp, Cavalerie de recunoaștere, General de armată. J2: 3× Brigadă cu pușcă, Tunuri de câmp, Cavalerie de recunoaștere, General de armată. Atac: 2d6 plus zarurile unității trebuie să atingă 5 plus ținta; la o diferență de 4 se adaugă o pagubă. Reguli speciale: moral (șocul slăbește unitățile; r reface moralul); imobilizare (focul la distanță încetinește ținta). Unități speciale: lideri. Câștigi distrugând toate unitățile inamice. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), r uN, end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
