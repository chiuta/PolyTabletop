# Litere (cuvinte pe tablă) (`litere`)

Categorie: Cuvinte. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play litere --seed 1`. Sursa implementării: `polytabletop-games/src/litere.rs`.

> Rezumat din `polytabletop list`: Joc original pe tablă 15×15 cu 97 de plăcuțe (2 blank): formezi cuvinte legate de cele existente, cu pătrate premiate (# cuvânt ×3, + cuvânt ×2, x literă ×3, - literă ×2); +50 cu toate cele 7 plăcuțe. Dicționar mic românesc încorporat (~4000 de cuvinte uzuale, fără diacritice; cuvintele absente sunt respinse). Mutări: casa@h8> (orizontal), casa@h8v (vertical), ca(s)a@h8> (blank între paranteze), schimb (toată mâna), pas. Primul cuvânt acoperă h8. Informație ascunsă: vezi doar mâna ta.

## Scop

Aduni cele mai multe puncte formând cuvinte pe tabla 15×15.

## Pregătire

97 de plăcuțe (2 blank `_`, care țin loc de orice literă și valorează 0); 7 plăcuțe fiecare. Primul cuvânt acoperă pătratul central h8 și are minimum două litere. Dicționar românesc mic (~4000 de cuvinte uzuale, scrise fără diacritice); cuvintele absente sunt respinse chiar dacă sunt corecte.

## Desfășurarea turei

Plasezi un cuvânt legat de cele existente. Toate cuvintele formate (principal și transversale) trebuie să fie în dicționar.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Orizontal: `casa@h8>`; vertical: `casa@h8v` (se scrie tot cuvântul, inclusiv literele deja de pe tablă; începutul la pătratul indicat).
* Blank între paranteze: `ca(s)a@h8>`.
* `schimb` (toată mâna; doar dacă în sac sunt cel puțin 7 plăcuțe), `pas`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (8): `bem@f8>`, `bem@g8>`, `bem@h8>`, `bem@h6v`, `bem@h7v`, `bem@h8v`, `schimb`, `pas`. Introduci `bem@f8>`; răspunsul calculatorului (în ordine): `volume@g3v`.
2. Mutări legale (35): `vi@g3>`, `vei@g3>`, `vii@g3>`, `voi@g3>`, `oi@g4>`, `foi@f4>`, `el@f5>`, `il@f5>`, `le@g5>`, `li@g5>` …. Introduci `iei@h9>`; răspunsul calculatorului (în ordine): `lada@j10>`.
3. Mutări legale (102): `vi@g3>`, `vei@g3>`, `voi@g3>`, `vor@g3>`, `vrei@g3>`, `vreo@g3>`, `oi@g4>`, `ou@g4>`, `foi@f4>`, `ore@g4>` …. Introduci `pas`; răspunsul calculatorului (în ordine): `pardo(n)@m9v`.

## Reguli speciale și variante alese

* Pătrate premiate: `#` cuvânt ×3, `+` cuvânt ×2, `x` literă ×3, `-` literă ×2.
* Diacriticele se elimină la încărcare (ă, â → a; î → i; ș → s; ț → t).

## Punctaj

Valoarea literelor × premiile de literă ale pătratelor noi, apoi premiile de cuvânt; +50 la folosirea tuturor celor 7 plăcuțe.

## Sfârșitul jocului

Cineva rămâne fără plăcuțe cu sacul gol (primește valoarea plăcuțelor adversarului), sau patru ture consecutive fără scor; se scade valoarea plăcuțelor rămase. Peste 400 de mutări jocul se încheie la fel.

## Note despre AI

Informație ascunsă: afișajul arată doar mâna ta. AI-ul nu se uită la mâna adversarului și ignoră la evaluare plăcuțele abia trase din sac. Programul alege automat unde folosește blankurile.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul NU este în `HOTSEAT_SAFE`: `--you both` afișează avertismentul „nu ascunde informația privată în modul la masă” (afișajul poate arăta mâna/informația Jucătorului 1), iar `hint` este dezactivat.

## Diferențe față de regulile tradiționale

* Dicționar mic, nu unul oficial: lipsesc multe forme flexionare.
* Set de litere, valori și pătrate premiate originale.
