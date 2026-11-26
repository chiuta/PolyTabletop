# Drumul stivelor (5×5) (`drumul_stivelor`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Variantă proprie, inspirată de tipul de joc „Tak”. Pornire: `polytabletop play drumul_stivelor --seed 1`. Sursa implementării: `polytabletop-games/src/drumul_stivelor.rs`.

> Reguli: Tablă 5×5; fiecare are 21 de pietre plate și o coroană. O tură: pui o piesă (f plată, s în picioare = zid, c coroană) sau muți o stivă condusă de tine: iei până la 5 piese de sus și le lași câte una sau mai multe pe celulele din direcția aleasă (n, s, e, v). Zidul oprește stivele, doar coroana singură îl turtește. Câștigi cu un drum de piese plate/coroane între margini opuse. Mutări: f c3, s c3, c c3, c3n3:21.
