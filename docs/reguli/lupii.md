# Lupii din Hârtop (roluri ascunse, masa = roboți) (`lupii`)

Lupii din Hârtop: joc de roluri ascunse (familia „lup și sat”). Tu ești locul 1 la o masă de 8–9; ceilalți sunt roboți.

## Scop

Satul: eliminați toți lupii. Lupii: ajungeți să fiți cel puțin cât restul.

## Pregătire

La început alegi presetarea: `preset mafie` (stil clasic: doi mafioți, doctor, detectiv), `preset sat` (stil francez: lupi, văzătoare, vrăjitoare, vânător), `preset stepa` (stil chinezesc: trei lupi, profet, paznic, vrăjitoare, vânător).

## Desfășurarea unei ture

Noaptea: lupii aleg o victimă; rolurile speciale acționează (`ucide 3`, `cerceteaza 3`, `pazeste 3`, `salveaza`, `otraveste 3`, `impusca 3`, `dormi`). Ziua: toți votează cine e scos din sat (`vot 3`); la egalitate se trage la sorți între cei egali; rolul celui eliminat se arată. Mutarea forțată `polytabletop` lasă roboții să acționeze.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `preset sat` — alegi presetarea
* `ucide 3` — lupii aleg victima
* `cerceteaza 3` — verifici un jucător
* `pazeste 3` — protejezi un jucător
* `salveaza / otraveste 3` — leac/otravă (vrăjitoare)
* `impusca 3` — vânătorul trage
* `dormi` — nu faci nimic
* `vot 3` — votezi pentru eliminare
* `polytabletop` — roboții acționează (forțat)

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play lupii --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `preset stepa` — alegi presetarea „stepa” (stil chinezesc, 9 jucători).
2. `pazeste 9` — ești Paznic: ocrotești locul 9.
3. `vot 4` — votezi eliminarea locului 4.

## Reguli speciale și variante alese

Roboții joacă după euristici și văd doar ce ar vedea un jucător real (propriul rol, ce s-a anunțat public, rolurile celor morți). Robotul „acuză” public o dată pe zi.

## Punctaj

Nu există scor.

## Sfârșitul jocului

Satul câștigă când nu mai e niciun lup; lupii câștigă când sunt cel puțin cât restul.

## Note despre AI și informația ascunsă

Informație ascunsă: rolurile; roboții (masa, `Player::Two`) nu văd rolurile tale. Vrăjitoarea poate folosi un singur leac pe noapte.

## Diferențe față de regulile tradiționale

Simplificări declarate: fără Cupidon/Primar, fără discuții libere (robotul „acuză” o dată pe zi), vrăjitoarea poate folosi un singur leac pe noapte.
