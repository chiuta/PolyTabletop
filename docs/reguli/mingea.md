# Mingea (Dodgeball) (`mingea`)

Două echipe de câte 4 jucători pe 7 coloane. Pe rând muți un jucător într-o coloană vecină liberă (`3>4`) sau arunci spre o coloană adversă (`arunc 4`). Mingea nu cade exact unde ai țintit: `zbor` hotărăște la întâmplare coloana (50% ținta, 20% fiecare vecină, 5% la două coloane). Cine e în coloana de aterizare iese. Câștigă echipa care rămâne cu jucători.

## Scop

Vezi regulile de mai sus; câștigă jucătorul care îndeplinește condiția de final a jocului.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Informație ascunsă: nu (zarurile/extragerile se fac la mutări forțate, iar AI-ul nu le vede în avans). Reguli din surse publice / domeniu public; unde există variante regionale s-a ales cea mai răspândită, iar simplificările sunt descrise în comentariul de modul din `polytabletop-games/src/mingea.rs`.

