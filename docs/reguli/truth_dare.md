# Truth or Dare (Adevăr sau provocare) (`truth_dare`)

Joc de petrecere pentru doi, familial. În 12 ture, jucătorul la rând alege `adevar` sau `provocare` (sau `refuz`, −1 punct); jocul arată întrebarea sau provocarea, el o execută, iar celălalt este judecător: `ok` (performer +1 la adevăr / +2 la provocare, judecător +1) sau `nu` (performer −1). Scorul mai mare câștigă.

## Scop

Vezi regulile de mai sus; câștigă jucătorul care îndeplinește condiția de final a jocului.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Informație ascunsă: nu (zarurile/extragerile se fac la mutări forțate, iar AI-ul nu le vede în avans). Reguli din surse publice / domeniu public; unde există variante regionale s-a ales cea mai răspândită, iar simplificările sunt descrise în comentariul de modul din `polytabletop-games/src/truth_dare.rs`.

