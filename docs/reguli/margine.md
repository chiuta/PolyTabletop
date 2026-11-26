# De la margine (hex 37) (`margine`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „GIPF”. Pornire: `polytabletop play margine --seed 1`. Sursa implementării: `polytabletop-games/src/margine.rs`.

> Reguli: Tablă hexagonală cu 37 de celule, 10 piese în rezervă pe jucător. Introduci o piesă de pe margine într-o direcție; dacă prima celulă e ocupată, tot șirul din linie e împins (ilegal dacă linia e plină). Un șir de 4+ piese ale aceluiași jucător se scoate: ale lui se întorc în rezervă, cele adverse din prelungire sunt pierdute. Pierzi când rămâi fără rezervă. Mutări: a1>se (intrare a1, direcția se); direcții e, ne, nv, v, sv, se.
