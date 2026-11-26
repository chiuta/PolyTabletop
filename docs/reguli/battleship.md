# Bătălia navală (10×10) (`battleship`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play battleship --seed 1`. Sursa implementării: `polytabletop-games/src/battleship.rs`.

> Rezumat din `polytabletop list`: Flotă de 5 nave (5, 4, 3, 3, 2). Îți așezi navele (a1h = orizontal, a1v = vertical, sau aleator); flota AI-ului se așază printr-o mutare forțată (plasez) și rămâne ascunsă. Tragi pe rând (e5): apă, lovit sau scufundat. AI-ul vânează cu hartă de densitate și țintește, fără să citească flota ta. Final cu mutarea forțată recunosc. Mutări: a1h, aleator, plasez, e5, recunosc.

## Scop

Scufunzi primul toată flota adversă.

## Pregătire

Flotă de 5 nave: portavion (5), cuirasat (4), crucișător (3), submarin (3), distrugător (2), orizontale sau verticale, fără suprapunere (se pot atinge). Îți așezi navele pe rând; flota AI-ului se așază printr-o mutare forțată (`plasez`) și rămâne ascunsă.

## Desfășurarea turei

Tragi câte o lovitură pe rând; rezultat: apă, lovit sau scufundat.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Plasare: `a1h` (nava pornește din a1 spre dreapta), `a1v` (în jos) sau `aleator`.
* `plasez` (forțată).
* Lovitură: `e5`.
* `recunosc` (forțată, cel învins confirmă).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (121): `a1h`, `a1v`, `b1h`, `b1v`, `c1h`, `c1v`, `d1h`, `d1v`, `e1h`, `e1v` …. Introduci `a1h`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (130): `f1h`, `f1v`, `g1h`, `g1v`, `h1v`, `i1v`, `j1v`, `a2h`, `a2v`, `b2h` …. Introduci `d5v`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (132): `f1h`, `f1v`, `g1h`, `g1v`, `h1h`, `h1v`, `i1v`, `j1v`, `a2h`, `a2v` …. Introduci `h10h`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

Nicio regulă specială dincolo de cele descrise mai sus.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Când ultima navă a cuiva e scufundată; cel învins confirmă prin `recunosc`, ca AI-ul să nu „vadă” loviturile câștigătoare.

## Note despre AI

Informație ascunsă: flota AI-ului nu apare în afișaj până la final. AI-ul vânează cu o hartă de densitate și țintește, calculând scorul înainte de a trage, doar din tabla lui de lovituri și din lungimile navelor nescufundate; nu citește flota ta.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Fără variante cu salve multiple.
