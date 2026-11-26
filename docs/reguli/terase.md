# Terase (construcție 5×5) (`terase`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Santorini”. Pornire: `polytabletop play terase --seed 1`. Sursa implementării: `polytabletop-games/src/terase.rs`.

> Reguli: Tablă 5×5, doi constructori pe jucător. O tură: muți un constructor pe o celulă vecină liberă cu cel mult un nivel mai sus, apoi construiești un nivel pe o celulă vecină (după nivelul 3 vine cupola care blochează). Câștigi urcând pe nivelul 3 sau lăsând adversarul fără tură. Mutări: b2-b3+c3 (mută pe b3, construiește pe c3), b2-b3 (câștig).
