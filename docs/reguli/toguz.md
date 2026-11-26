# Toguz Kumalak (Kazahstan) (`toguz`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play toguz --seed 1`. Sursa implementării: `polytabletop-games/src/toguz.rs`.

> Rezumat din `polytabletop list`: 2 × 9 gropi cu câte 9 bile și un depozit (kazan) pentru fiecare. Iei toate bilele dintr-o groapă proprie: prima rămâne, restul se seamănă câte una în sens trigonometric (o singură bilă trece mai departe). Dacă ultima bilă face o groapă a adversarului pară, o iei; dacă o face 3 (nu groapa 9, una per jucător, nu cu același număr ca a adversarului), groapa devine tuzdyk: orice bilă semănată acolo ajunge în depozitul tău. Primul la 82 câștigă. Mutări: 1–9.

## Reguli implementate

2 × 9 gropi cu câte 9 bile și un depozit (kazan) pentru fiecare. Iei toate bilele dintr-o groapă proprie: prima rămâne, restul se seamănă câte una în sens trigonometric (o singură bilă trece mai departe). Dacă ultima bilă face o groapă a adversarului pară, o iei; dacă o face 3 (nu groapa 9, una per jucător, nu cu același număr ca a adversarului), groapa devine tuzdyk: orice bilă semănată acolo ajunge în depozitul tău. Primul la 82 câștigă. Mutări: 1–9.

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
