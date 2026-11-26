# Ligretto (`ligretto`)

Jocul de cărți rapid, pe ture pentru doi. Fiecare are o stivă Ligretto de 10 cărți, 3 rânduri și o mână de 27; pe masă se construiesc grămezi ascendente pe culoare de la 1 la 10. Tura: câte mutări vrei (`l-c2`, `w-r1`, `r2-c1`, `l-cn` pentru un 1 nou), apoi `da` întoarce 3 cărți din mână. Pe rânduri pui cu 1 mai mic și altă culoare. Câștigă cine golește Ligretto; altfel punctele (cărți puse − 2× cărți rămase).

## Scop

Vezi regulile de mai sus; câștigă jucătorul care îndeplinește condiția de final a jocului.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Informație ascunsă: nu (zarurile/extragerile se fac la mutări forțate, iar AI-ul nu le vede în avans). Reguli din surse publice / domeniu public; unde există variante regionale s-a ales cea mai răspândită, iar simplificările sunt descrise în comentariul de modul din `polytabletop-games/src/ligretto.rs`.

