# Perquackey (`perquackey`)

Joc de cuvinte cu 15 zaruri-litere, pe ture pentru doi. Zarurile se aruncă la începutul fiecărei runde (`arunc`); apoi pe rând propui un cuvânt de cel puțin 3 litere format din zaruri (`cuv casa`, fiecare zar o dată, un cuvânt doar o dată pe rundă) sau `pas`. Două `pas` la rând încheie runda. Punctaj: lungimea minus 2. 3 runde; scorul mai mare câștigă.

## Scop

Vezi regulile de mai sus; câștigă jucătorul care îndeplinește condiția de final a jocului.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Informație ascunsă: nu (zarurile/extragerile se fac la mutări forțate, iar AI-ul nu le vede în avans). Reguli din surse publice / domeniu public; unde există variante regionale s-a ales cea mai răspândită, iar simplificările sunt descrise în comentariul de modul din `polytabletop-games/src/perquackey.rs`.

