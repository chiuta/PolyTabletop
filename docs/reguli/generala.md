# Generala (zaruri) (`generala`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play generala --seed 1`. Sursa implementării: `polytabletop-games/src/generala.rs`.

> Rezumat din `polytabletop list`: 5 zaruri, până la 3 aruncări pe tură, 10 categorii: unu…sase (suma zarurilor), escalera 20, full 30, poker 40, generala 50. Servida (din prima aruncare) +5 la escalera/full/poker; generala servida câștigă jocul pe loc. Categoriile nepotrivite se taie (0). 10 ture pe jucător, câștigă totalul mai mare. Mutări: arunc, tine 3 3 5, tine -, scor escalera.

## Reguli implementate

5 zaruri, până la 3 aruncări pe tură, 10 categorii: unu…sase (suma zarurilor), escalera 20, full 30, poker 40, generala 50. Servida (din prima aruncare) +5 la escalera/full/poker; generala servida câștigă jocul pe loc. Categoriile nepotrivite se taie (0). 10 ture pe jucător, câștigă totalul mai mare. Mutări: arunc, tine 3 3 5, tine -, scor escalera.

## Variante alese și limite

Categoriile sunt cele 10 clasice; la 3 aruncări poți păstra orice zaruri (`tine`). Fără «dublă generala». Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație completă (fără cărți ascunse).

## Calculatorul

Adversarul AI caută în arbore (alpha-beta) cu o funcție de evaluare care nu citește informația ascunsă; nu trișează.
