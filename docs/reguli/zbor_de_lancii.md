# Zbor de Lănci (`zbor_de_lancii`)

Categorie: Miniaturi. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play zbor_de_lancii --seed 1`. Sursa implementării: `polytabletop-games/src/t4_skirm.rs` și `t4_sdata.rs`.

> Rezumat din `polytabletop list`: Variantă originală pentru doi pe o tablă 12×8 pătrată (8 direcții) cu teren deschis. Punctele de acțiune sunt jumătate din unitățile tale vii (2–5); fiecare acțiune costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Cavalerie grea 4/6/1/4/5; Pedestrime cu scut 3/3/1/3/4; Arcași montați 2/6/4/2/3; Comandant 3/5/1/3/4. J1: 2× Cavalerie grea, 2× Pedestrime cu scut, Arcași montați, Comandant. J2: 2× Cavalerie grea, 2× Pedestrime cu scut, Arcași montați, Comandant. Atac: 2d6 plus zarurile unității trebuie să atingă 5 plus ținta; la o diferență de 4 se adaugă o pagubă. Reguli speciale: asalt (+1 zar după o mutare de 2+ pătrate); flancare (+1 zar dacă un aliat e lângă țintă). Unități speciale: lideri; trăgători mobili. Controlează punctele + (mai multe unități la cel mult 1 pas); fiecare rundă dă un punct pentru fiecare punct ținut; primul la 9 câștigă. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Despre variantă

Joc original (joc de miniaturi pe tablă), scris „clean-room”: nu folosește numele, textul regulilor, componentele, hărțile, personajele sau cărțile vreunui produs comercial; se inspiră doar din mecanica generală a genului. Titlul și regulile sunt proprii. Intrarea din catalog: «impetus».

## Reguli

Variantă originală pentru doi pe o tablă 12×8 pătrată (8 direcții) cu teren deschis. Punctele de acțiune sunt jumătate din unitățile tale vii (2–5); fiecare acțiune costă 1. Unități (viață/mutare/bătaie/zaruri/țintă): Cavalerie grea 4/6/1/4/5; Pedestrime cu scut 3/3/1/3/4; Arcași montați 2/6/4/2/3; Comandant 3/5/1/3/4. J1: 2× Cavalerie grea, 2× Pedestrime cu scut, Arcași montați, Comandant. J2: 2× Cavalerie grea, 2× Pedestrime cu scut, Arcași montați, Comandant. Atac: 2d6 plus zarurile unității trebuie să atingă 5 plus ținta; la o diferență de 4 se adaugă o pagubă. Reguli speciale: asalt (+1 zar după o mutare de 2+ pătrate); flancare (+1 zar dacă un aliat e lângă țintă). Unități speciale: lideri; trăgători mobili. Controlează punctele + (mai multe unități la cel mult 1 pas); fiecare rundă dă un punct pentru fiecare punct ținut; primul la 9 câștigă. Cel mult 12 runde, apoi decide scorul. Mutări: m uN d4 (mută), a uN uM (atacă), end.

## Informație ascunsă și AI

Informația este deschisă; zarurile se aruncă la aplicarea mutării cu un generator re-însămânțat la clonare, deci AI-ul nu vede aruncările viitoare și nu trișează. Adâncimea căutării este fixă (nu depinde de ceas); nivelurile Easy/Normal/Hard/Expert scalează adâncimea.
