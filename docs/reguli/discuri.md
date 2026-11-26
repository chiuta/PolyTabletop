# Discuri (fotbal pe ture) (`discuri`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Subbuteo”. Pornire: `polytabletop play discuri --seed 1`. Sursa implementării: `polytabletop-games/src/discuri.rs`.

> Reguli: Teren 9×13, porți late de 3 celule. Fiecare echipă are 5 discuri; mingea e la centru. O tură: alegi un disc propriu, o direcție (8) și o putere 1–3: discul alunecă până la atâtea celule, oprindu-se înaintea altui disc sau a marginii; dacă drumul ajunge la minge, discul se oprește lângă ea și mingea e lovită în aceeași direcție cu atâtea celule cât puterea. Mingea ieșită prin poartă e gol. Primul la 2 goluri câștigă; după 160 de semi-mutări, cine are mai multe. Mutări: c3ne2 (discul c3, direcția ne, puterea 2).
