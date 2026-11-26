# Scaunele muzicale (`scaune_muzicale`)

Doi dansatori pe un cerc de 8 locuri, cu un singur scaun marcat # în fiecare rundă. Pe rând faci un pas de 1, 2 sau 3 locuri (`pas1`, `pas2`, `pas3`; nu aterizezi pe adversar, nu poți sta pe loc). După fiecare mutare `asculta` poate opri muzica (1 șansă din 5): cine stă atunci pe scaun ia punctul rundei. Primul la 3 puncte câștigă.

## Scop

Vezi regulile de mai sus; câștigă jucătorul care îndeplinește condiția de final a jocului.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Informație ascunsă: nu (zarurile/extragerile se fac la mutări forțate, iar AI-ul nu le vede în avans). Reguli din surse publice / domeniu public; unde există variante regionale s-a ales cea mai răspândită, iar simplificările sunt descrise în comentariul de modul din `polytabletop-games/src/scaune_muzicale.rs`.

