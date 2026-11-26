# Dame internaționale (10×10) (`dame10`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play dame10 --seed 1`. Sursa implementării: `polytabletop-games/src/dame10.rs`.

> Rezumat din `polytabletop list`: Tablă 10×10, 20 de piese pe jucător. Piesele simple mută un pas în diagonală înainte și capturează înainte și înapoi; damele zboară pe orice distanță. Captura e obligatorie, cu cele mai multe piese; piesele luate se scot la final. O piesă devine damă doar dacă își încheie mutarea pe ultimul rând. Remiză după 50 de plie doar cu dame sau 400 în total. Mutări: c4-d5, c4xe6xg8.

## Reguli implementate

Tablă 10×10, 20 de piese pe jucător. Piesele simple mută un pas în diagonală înainte și capturează înainte și înapoi; damele zboară pe orice distanță. Captura e obligatorie, cu cele mai multe piese; piesele luate se scot la final. O piesă devine damă doar dacă își încheie mutarea pe ultimul rând. Remiză după 50 de plie doar cu dame sau 400 în total. Mutări: c4-d5, c4xe6xg8.

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
