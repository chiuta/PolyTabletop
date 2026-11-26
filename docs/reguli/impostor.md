# Impostorul din Grilă (4 locuri) (`impostor`)

Impostorul din Grilă: joc cu impostor (un „cameleon”) pe patru locuri: Jucătorul 1, Jucătorul 2 și doi roboți de masă (Robot A și Robot B).

## Scop

Ca nevinovat, prinde impostorul; ca impostor, nu fi prins sau ghicește cuvântul secret.

## Pregătire

Pe masă e o grilă de 16 cuvinte dintr-un subiect (14 subiecte × 16 cuvinte). Un cuvânt este secret și toți îl știu, mai puțin impostorul (ales la întâmplare dintre cei patru; îți vezi rolul în afișaj).

## Desfășurarea unei ture

Pe rând, fiecare spune un indiciu dintr-un singur cuvânt. Cine știe cuvântul alege dintre indiciile lui (`indiciu rosie`); impostorul alege la nimereală un indiciu „de trecut” din grilă, ascultând ce s-a spus. Un indiciu nu se repetă. Apoi J1 și J2 votează (`vot N`: 1 = J1, 2 = J2, 3 = Robot A, 4 = Robot B); roboții votează în pasul forțat `numara`. Cine are cele mai multe voturi (fără egalitate) e acuzat. Dacă impostorul e prins, are o ultimă șansă să ghicească cuvântul (`ghici cuvant`, apoi `verifica`).

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `indiciu rosie` — spui un indiciu dintr-un singur cuvânt (fără diacritice), din lista oferită
* `vot 2` — votezi locul 2 ca impostor
* `ghici cuvant` — impostorul prins ghicește cuvântul
* `numara` — pas forțat: votează roboții
* `verifica` — pas forțat: verifică ghicirea

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play impostor --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `indiciu rosie` — spui indiciul „roșie” (știi cuvântul secret CIREAȘĂ).
2. `vot 2` — votezi locul 2 (J2) ca impostor.
3. `indiciu rotire` — în runda 2 ești impostor și spui indiciul „rotire”.

## Reguli speciale și variante alese

Indiciile se scriu fără diacritice.

## Punctaj

Puncte (doar J1 și J2): fiecare om care a votat impostorul +1; impostor neprins +2; impostor prins care nu ghicește: fiecare om nevinovat +2; prins care ghicește: +1 pentru impostor.

## Sfârșitul jocului

După 8 runde, mai multe puncte câștigă.

## Note despre AI și informația ascunsă

AI-ul folosește numai ce știe rolul lui: cine știe cuvântul se uită la indicii, iar impostorul deduce cuvântul din indiciile spuse (nu citește cuvântul secret).

## Diferențe față de regulile tradiționale

Variantă originală; cuvintele sunt ale noastre.
