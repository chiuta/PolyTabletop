# Chinchón (Spania) (`chinchon`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play chinchon --seed 1`. Sursa implementării: `polytabletop-games/src/chinchon.rs`.

> Rezumat din `polytabletop list`: Baraja spaniolă de 40 de cărți (o c e b; 1–7, 10, 11, 12), 7 cărți fiecare. Tragi din stoc sau talon și arunci; combinații: seturi și secvențe (7 nu se leagă de 10). Închizi cu cel puțin 6 cărți potrivite și cel mult 5 puncte libere (−10 dacă toate 7 se potrivesc; chinchón = 1–7 din aceeași culoare câștigă jocul); cine ajunge la 100 pierde. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: stoc, talon, 5o, inchide-5o, urmatoarea.

## Reguli implementate

Baraja spaniolă de 40 de cărți (o c e b; 1–7, 10, 11, 12), 7 cărți fiecare. Tragi din stoc sau talon și arunci; combinații: seturi și secvențe (7 nu se leagă de 10). Închizi cu cel puțin 6 cărți potrivite și cel mult 5 puncte libere (−10 dacă toate 7 se potrivesc; chinchón = 1–7 din aceeași culoare câștigă jocul); cine ajunge la 100 pierde. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: stoc, talon, 5o, inchide-5o, urmatoarea.

## Variante alese și limite

Fără jokeri și fără pachetul de 48 de cărți. Chinchón (1–7 din aceeași culoare) câștigă jocul pe loc. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: în modul hotseat ecranul se ascunde între jucători.

## Calculatorul

Adversarul AI caută în arbore (alpha-beta) cu o funcție de evaluare care nu citește informația ascunsă; nu trișează.
