# Uckers (`uckers`)

Ludo marinăresc britanic: patru piese pe jucător, un singur zar, piesele ies doar cu 6 și fac un tur de 48 de pătrate, apoi urcă pe coloana de casă. Două piese proprii pe același pătrat formează un bloc (spate-în-spate) care nu poate fi capturat. Un 6 sau o captură dă o aruncare în plus; câștigă cine ajunge primul cu toate piesele acasă.

## Scop

Vezi regulile de mai sus; câștigă jucătorul care îndeplinește condiția de final a jocului.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

## Reguli speciale și variante alese

Informație ascunsă: nu (zarurile/extragerile se fac la mutări forțate, iar AI-ul nu le vede în avans). Reguli din surse publice / domeniu public; unde există variante regionale s-a ales cea mai răspândită, iar simplificările sunt descrise în comentariul de modul din `polytabletop-games/src/uckers.rs`.

