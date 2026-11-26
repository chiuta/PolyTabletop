# Duck Duck Goose (`duckduckgoose`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play duckduckgoose --seed 1`. Sursa implementării: `polytabletop-games/src/duckduckgoose.rs`.

> Rezumat din `polytabletop list`: Joc de cerc pentru copii, ca duel de risc pentru doi: în fiecare rundă un jucător atinge, celălalt e gâsca. Cel care atinge alege câte rațe spune (1–5): cu cât mai multe, cu atât are șanse mai mici să fie prins și mai multe puncte dacă scapă. Cursa se hotărăște la pasul forțat fuga; 10 runde, câștigă cine are mai multe puncte. Mutări: rate 1…rate 5, fuga.

## Reguli implementate

Joc de cerc pentru copii, ca duel de risc pentru doi: în fiecare rundă un jucător atinge, celălalt e gâsca. Cel care atinge alege câte rațe spune (1–5): cu cât mai multe, cu atât are șanse mai mici să fie prins și mai multe puncte dacă scapă. Cursa se hotărăște la pasul forțat fuga; 10 runde, câștigă cine are mai multe puncte. Mutări: rate 1…rate 5, fuga.

## Variante alese și limite

Adaptare ca duel de risc pentru doi (nu joc de cerc); cursa se hotărăște la pasul forțat `fuga`. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
