# Orașul Detectivilor (urmărirea suspecților pe hartă) (`detectiv`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play detectiv --seed 1`. Sursa implementării: `polytabletop-games/src/detectiv.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: grilă 6×6, 6 suspecți, 8 pași de timp; crima la pasul 5 în celula X. arata s,t dezvăluie public poziția suspectului s la pasul t (apoi afisez), acuz s acuză (apoi verific): corect câștigi, greșit pierzi. Suspecții care nu puteau ajunge în X sunt eliminați. Informație ascunsă: traseele. Mutări: arata s,t, afisez, acuz s, verific.
