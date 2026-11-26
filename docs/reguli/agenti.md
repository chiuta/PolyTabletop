# Agenții Tăcuți (agenți pe o stradă cu valori secrete) (`agenti`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play agenti --seed 1`. Sursa implementării: `polytabletop-games/src/agenti.rs`.

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală: 6 agenți (A–F) pe o stradă de 10 pași; fiecare jucător are în secret valori distincte 0–5. Pe rând muți orice agent cu 1–3 (mut X,k). După 24 de mutări scorul = Σ valoarea ta × poziția. Informație ascunsă: vezi doar valorile tale. Câștigă scorul mai mare. Mutări: mut X,k.
