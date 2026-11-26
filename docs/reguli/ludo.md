# Nu te supăra, frate (Ludo) (`ludo`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play ludo --seed 1`. Sursa implementării: `polytabletop-games/src/ludo.rs`.

> Rezumat din `polytabletop list`: 4 piese, un zar. Cu 6 scoți o piesă din bază; 6 = încă o aruncare; aterizarea pe adversar îl trimite în bază; ținta se atinge exact. Mutări: intră, sau progresul piesei (ex. 12).

## Scop

Aduci primul toate cele 4 piese în țintă.

## Pregătire

Pistă comună de 40 de pătrate; Unu pornește de la pătratul 0, Doi de la 20. Fiecare piesă are un „progres”: 0 = în bază, 1–40 = pe pistă, 41–43 = coloana de acasă, 44 = ținta.

## Desfășurarea turei

Zarul se aruncă automat; rezultatul apare în afișaj. Muți o piesă cu atâtea pătrate. Un 6 scoate o piesă din bază și dă dreptul la încă o aruncare. Fără piese pe tablă ai trei încercări de a arunca 6. Dacă nu ai nicio mutare, tura trece automat.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `intră` (scoate o piesă din bază).
* Numărul progresului piesei pe care o muți, de exemplu `12`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (1): `intră`. Introduci `intră`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (1): `1`. Introduci `1`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (2): `intră`, `7`. Introduci `7`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Nu poți ateriza pe o piesă proprie; ținta se atinge exact.
* Aterizarea pe o piesă adversă de pe pista comună o trimite în bază; coloana de acasă nu poate fi capturată.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Când toate cele 4 piese ale unui jucător au ajuns în țintă.

## Note despre AI

Zarul este informație publică. AI alpha-beta de adâncime 1 pe progresul total; nu vede zarurile viitoare.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`; are elemente aleatoare, dar acestea se consideră publice pentru categoria Zaruri, deci `--you both` afișează avertisment, dar `hint` rămâne disponibil.

## Diferențe față de regulile tradiționale

* Doi jucători, pistă de 40 de pătrate.
