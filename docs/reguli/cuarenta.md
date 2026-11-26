# Cuarenta (Ecuador) (`cuarenta`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play cuarenta --seed 1`. Sursa implementării: `polytabletop-games/src/cuarenta.rs`.

> Rezumat din `polytabletop list`: Baraja spaniolă de 40 de cărți, câte 5 în mână, masa începe goală. Joci o carte și poți lua: o carte de același rang (potrivire) sau cărți 1–7 cu aceeași sumă (adunare), plus secvența ascendentă care urmează. Caída 2, limpia 2, ronda 4 puncte; la capăt de mână puncte după numărul de cărți luate; primul la 40. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: 5o, 5o:5c, 6o:2c+4e, imparte, urmatoarea.

## Reguli implementate

Baraja spaniolă de 40 de cărți, câte 5 în mână, masa începe goală. Joci o carte și poți lua: o carte de același rang (potrivire) sau cărți 1–7 cu aceeași sumă (adunare), plus secvența ascendentă care urmează. Caída 2, limpia 2, ronda 4 puncte; la capăt de mână puncte după numărul de cărți luate; primul la 40. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: 5o, 5o:5c, 6o:2c+4e, imparte, urmatoarea.

## Variante alese și limite

Fără recunoașterea rondei ca mutare separată; secvența ascendentă se ia automat. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: în modul hotseat ecranul se ascunde între jucători.

## Calculatorul

Adversarul AI caută în arbore (alpha-beta) cu o funcție de evaluare care nu citește informația ascunsă; nu trișează.
