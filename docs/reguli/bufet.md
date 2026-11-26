# Bufet (variantă originală) (`bufet`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play bufet --seed 1`. Sursa implementării: `polytabletop-games/src/bufet.rs`.

> Rezumat din `polytabletop list`: Draft rapid de 10 cărți, 3 runde: alegi o carte (alege mici), mâinile se schimbă după revelare (revelare). Mici (perechi 5), salată (triplete 10), colțunași (1/3/6/10/15), cârnați 1-2-3 (muștarul îi triplează), pateuri (majoritate 6/3), papanași (la final cei mai mulți +6, cei mai puțini −6). Alegerea adversarului rămâne ascunsă până la revelare.

## Scop

Aduni cele mai multe puncte din 3 runde de draft, combinând cărți de tip Mici, Salată, Colțunași, Cârnați, Pateuri și Papanași.

## Pregătire

Fiecare rundă: 10 cărți în mână; alegeți simultan, apoi mâinile se schimbă. Simultaneitatea este simulată: Jucătorul 1 alege în secret, apoi Jucătorul 2, apoi mutarea forțată `revelare` dezvăluie ambele alegeri.

## Desfășurarea turei

Alegi o carte din mână; după ce ambii au ales, `revelare` arată alegerile, cărțile se pun în fața voastră și mâinile se schimbă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `alege mici`, `alege salata`, `alege coltunasi`, `alege carnat1` (…`carnat3`), `alege mustar`, `alege pateu3`, `alege papanas`.
* `revelare` (forțată).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (8): `alege salata`, `alege coltunasi`, `alege carnat1`, `alege carnat2`, `alege carnat3`, `alege mustar`, `alege pateu2`, `alege pateu3`. Introduci `alege salata`; răspunsul calculatorului (în ordine): `alege pateu2`.
2. Mutări legale (1): `revelare`. Introduci `revelare`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (7): `alege mici`, `alege salata`, `alege carnat1`, `alege carnat2`, `alege pateu1`, `alege pateu2`, `alege papanas`. Introduci `alege papanas`; răspunsul calculatorului (în ordine): `alege carnat3`.

## Reguli speciale și variante alese

* Mici: fiecare pereche = 5 puncte. Salată: fiecare triplet = 10.
* Colțunași: 1, 3, 6, 10, 15 puncte pentru 1–5+ cărți.
* Cârnați de 1/2/3 puncte; Muștarul triplează următorul cârnat jucat.
* Pateuri (1/2/3 pictograme): cele mai multe pictograme într-o rundă = 6, locul 2 = 3 (egalitate: 6 împărțit).
* Papanași: la final, cei mai mulți +6, cei mai puțini −6.

## Punctaj

Suma punctelor de mai sus pe cele 3 runde.

## Sfârșitul jocului

După 3 runde câștigă cine are mai multe puncte.

## Note despre AI

Alegerea adversarului rămâne ascunsă până la `revelare`. AI-ul nu citește alegerea ascunsă a adversarului și nici mâna lui.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Variantă originală: draft rapid cu teme și valori proprii.
