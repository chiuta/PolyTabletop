# Hanafuda (Koi-Koi) (`hanafuda`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play hanafuda --seed 1`. Sursa implementării: `polytabletop-games/src/hanafuda.rs`.

> Rezumat din `polytabletop list`: 48 de cărți de flori (12 luni × 4), 8 în mână și 8 pe masă; potrivești luna: obligatoriu capturezi dacă ai pereche (doi candidați = alegi: 3a:3c, trei = iei toate). Apoi întorci o carte din stoc (intoarce) și o joci la fel. Yaku nou (luminoase, mistreț-cerb-fluturi, panglici, animale, simple, hanami, tsukimi): koi-koi (continui) sau shobu (încasezi; ×2 la 7+ și ×2 dacă adversarul a spus koi-koi). 3 runde. Mutări: 3b, 3a:3c, intoarce, ia 3c, koi-koi, shobu.

## Scop

Faci yaku-uri (combinații) și încasezi punctele la 3 runde; câștigă cine are mai multe puncte.

## Pregătire

48 de cărți de flori (12 luni × 4), notate lună + literă: `3b` = a doua carte a lunii martie. 8 cărți în mână, 8 pe masă, restul stoc. Pachet: 5 luminoase (20), 9 animale (10), 10 panglici (5) și 24 simple (1).

## Desfășurarea turei

(1) Joci o carte din mână: dacă are pereche pe masă (aceeași lună) trebuie să captureze (doi candidați = alegi, trei = le iei pe toate); altfel rămâne pe masă. (2) Mutarea forțată `intoarce` întoarce prima carte din stoc și o joacă la fel (`ia 3c` dacă alegerea e între doi candidați). La un yaku nou alegi `koi-koi` sau `shobu`.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Carte: `3b`; captură la alegere: `3a:3c`.
* `intoarce` (forțată), `ia 3c`.
* `koi-koi` (continui), `shobu` (închizi runda și încasezi).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (8): `1b`, `3d`, `4c`, `4d`, `6b`, `6d`, `9c`, `11d`. Introduci `1b`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (1): `intoarce`. Introduci `intoarce`; răspunsul calculatorului (în ordine): `12b:12a`; `intoarce`.
3. Mutări legale (7): `3d`, `4c`, `4d`, `6b`, `6d`, `9c`, `11d`. Introduci `11d`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Yaku: Cinci luminoase 10, Patru luminoase 8, Patru luminoase cu ploaie 7, Trei luminoase 5, Ino-Shika-Chō 5 (+1 pe animal în plus), Tane 1 (+1), Akatan 5, Aotan 5, Tan 1 (+1), Kasu 1 (+1), Hanami 5, Tsukimi 5.
* Scorul se dublează la 7+ puncte și încă o dată dacă adversarul a spus koi-koi.

## Punctaj

Punctele yaku-urilor la `shobu`, cu dublările de mai sus.

## Sfârșitul jocului

După 3 runde câștigă cine are mai multe puncte. Dacă mâinile se golesc fără yaku, runda e remiză.

## Note despre AI

Informație ascunsă (mâna adversarului și stocul). Stocul întors nu e vizibil AI-ului dinainte, fiindcă `intoarce` este o mutare separată. AI-ul spune întotdeauna `shobu`.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Yaku-urile se verifică la sfârșitul turei (nu și după cartea din mână).
* Fără mâini speciale la împărțire.
