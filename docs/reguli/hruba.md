# Hoinari prin Hrubă (monștri și comori, masa = roboți) (`hruba`)

Hoinari prin Hrubă: joc umoristic de cărți cu monștri, niveluri și comori. Trei hoinari: tu și doi roboți.

## Scop

Fii primul care ajunge la nivelul 10 omorând un monstru.

## Pregătire

Fiecare începe cu nivel 1 și 4 cărți. Pachet de uși: monștri, blesteme, obiecte.

## Desfășurarea unei ture

Deschizi ușa (`deschide`): un monstru te atacă; un blestem se aplică pe loc; altceva intră în mână. Luptă: forța ta = nivel + armă + armură + poțiuni de forță; câștigi dacă forța ta DEPĂȘEȘTE nivelul monstrului (egal = pierzi). Fugă (`fugi`): zar d6, 5+ scapi, altfel pățești „păcatul” monstrului. Dacă nu a apărut monstru poți căuta (`cauta`) sau juca un monstru din mână (`vaneaza monstru4`). La sfârșitul turei ții maxim 5 cărți.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `deschide` — deschizi ușa
* `cauta` — tragi o carte de ușă în mână
* `echipeaza arma3` — echipezi arma 3
* `vaneaza monstru4` — joci un monstru din mână
* `crestere` — crești
* `forta` — folosești o poțiune de forță (+3)
* `lupta` — lupți
* `fugi` — fugi
* `scapa blestem` — scapi de un blestem
* `polytabletop` — roboții acționează (forțat)

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play hruba --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `deschide` — deschizi ușa: apare Cerberul de Apartament (6).
2. `forta` — folosești o poțiune de forță (+3).
3. `forta` — folosești a doua poțiune de forță (+3); acum urmează `lupta` sau `fugi`.

## Reguli speciale și variante alese

Roboții pot îngreuna lupta ta cu un monstru din mâna lor. Câștig: 1 nivel (2 la monștrii mari) și comori. Limită de 90 de ture; atunci câștigă cel mai mare nivel.

## Punctaj

Nivelul fiecărui hoinar (afișat în listă).

## Sfârșitul jocului

Primul la nivelul 10 omorând un monstru; sau cel mai mare nivel după 90 de ture.

## Note despre AI și informația ascunsă

Roboții folosesc doar informația publică.

## Diferențe față de regulile tradiționale

Simplificări declarate: fără clase/rase, fără cereri de ajutor, fără negocieri; limită de 90 de ture.
