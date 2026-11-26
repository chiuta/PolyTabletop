# Cărți colecționabile: pitch, lanț și echipament (Lanțuri de Oțel) (`tcg_lant`)

Cărți colecționabile cu pitch, lanț de răspuns și echipament („Seria Lanțurilor de Oțel”), 2 jucători.

## Scop

Aduci adversarul la 0 viață sau ai mai multă viață după 25 de ture pe jucător.

## Pregătire

Fiecare erou are 30 de viață, mână de 4 cărți, patru piese de echipament (cap, piept, brațe, picioare) și o armă; pachet pregătit de 30 de cărți (J1 „Gardiana Zorilor”, J2 „Vânătorul de Furtuni”). Resursele vin din pitch: costul unei cărți se plătește aruncând în zona de pitch cărți din mână a căror valoare însumează cel puțin costul.

## Desfășurarea unei ture

`pesca` (forțat: completezi mâna până la 4), apoi ai 1 punct de acțiune: `atac 2 pitch 1,3`, `arma pitch 2` (o dată pe tură), `joaca 3 pitch 1` (acțiune fără atac, nu costă punctul), `fin`. Un atac cu „continuă” dă înapoi punctul de acțiune. Lanț: apărătorul alege `bloc 1,3 echip 2` (cel mult 3 cărți și piese de echipament) sau `bloc fara`; atacatorul poate juca reacții de atac (`reactie 2 pitch 1`) până spune `pasa`; apoi apărătorul reacții de apărare. Daune = putere − apărare.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `pesca` — completezi mâna (forțat)
* `atac 2 pitch 1,3` — atac cu cartea 2, plătit cu cărțile 1 și 3
* `arma pitch 2` — atacul armei
* `joaca 3 pitch 1` — joci o acțiune
* `bloc 1,3 echip 2` — aperi cu cărți și echipament
* `bloc fara` — nu aperi
* `reactie 2 pitch 1` — reacție
* `pasa` — nu mai reacționezi
* `fin` — încheie tura

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play tcg_lant --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `pesca` — tragi o carte (forțat).
2. `joaca 2` — joci „Cumpătare” (acțiune cost 0: următorul atac +3).
3. `fin` — încheie tura.

## Reguli speciale și variante alese

La sfârșitul turei cărțile din pitch merg pe FUNDUL pachetului; surplusul de pitch se pierde. Fiecare piesă de echipament folosită pierde o folosire și dispare la zero.

## Punctaj

Viață, 30 la început.

## Sfârșitul jocului

Câștigă cine aduce adversarul la 0 viață; după 25 de ture pe jucător, cine are mai multă viață (egal = remiză).

## Note despre AI și informația ascunsă

`render` arată doar mâna J1; AI-ul (alpha-beta adâncime 1) citește doar câmpul, echipamentul și NUMĂRUL cărților adverse; apărarea adversarului se estimează din echipamentul vizibil.

## Diferențe față de regulile tradiționale

Set original, reguli scrise din mecanici generice.
