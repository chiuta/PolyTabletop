# Gebeta (Etiopia) (`gebeta`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play gebeta --seed 1`. Sursa implementării: `polytabletop-games/src/gebeta.rs`.

> Rezumat din `polytabletop list`: 2 × 6 gropi cu câte 4 semințe, câte un depozit. Iei toate semințele dintr-o groapă proprie și le semeni în sens trigonometric. Dacă ultima cade într-o groapă ocupată, ridici tot ce e acolo și continui (lanț). Dacă cade într-o groapă goală a ta și cea opusă are semințe, iei semințele opuse plus a ta. Când o parte rămâne goală jocul se termină și fiecare își ia semințele de pe partea lui; câștigă cine are mai multe. Mutări: 1–6.

## Reguli implementate

2 × 6 gropi cu câte 4 semințe, câte un depozit. Iei toate semințele dintr-o groapă proprie și le semeni în sens trigonometric. Dacă ultima cade într-o groapă ocupată, ridici tot ce e acolo și continui (lanț). Dacă cade într-o groapă goală a ta și cea opusă are semințe, iei semințele opuse plus a ta. Când o parte rămâne goală jocul se termină și fiecare își ia semințele de pe partea lui; câștigă cine are mai multe. Mutări: 1–6.

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
