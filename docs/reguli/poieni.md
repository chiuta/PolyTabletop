# Poieni (variantă originală) (`poieni`)

Poieni: plăcuțe hexagonale de habitat și jetoane de animale, variantă originală pentru doi.

## Scop

Ai scorul mai mare după 20 de ture pe jucător.

## Pregătire

Fiecare jucător își construiește un ținut din plăcuțe hexagonale (plus 3 plăcuțe de start fiecare). Fiecare muchie are un habitat: Pădure (P), Munte (M), Râu (R), Luncă (L), Mlaștină (B). Animale: Urs (U), Vidră (V), Zimbru (Z), Gaiță (G), Vulpe (F).

## Desfășurarea unei ture

Iei din piață o pereche plăcuță + jeton (`ia 2`), sau, cu un jeton-natură, orice plăcuță cu orice jeton (`nat 2 3`) ori schimbi cele 4 jetoane (`nat reset`); așezi plăcuța lipită de ținutul tău, în orice rotire 0–5 (`pune 1,-1 r3`); apoi pui jetonul pe o plăcuță liberă care îl acceptă (`animal 0,1`) sau îl arunci (`arunca`).

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `ia 1` — iei perechea 1 din piață
* `pune -1,1 r3` — așezi plăcuța la coordonatele q,r cu rotirea 3
* `animal 0,1` — pui jetonul pe plăcuța de la 0,1
* `arunca` — arunci jetonul
* `nat 2 3` — cu un jeton-natură: plăcuța 2 cu jetonul 3
* `nat reset` — schimbi cele 4 jetoane

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play poieni --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `ia 1` — iei perechea 1 din piață.
2. `pune -1,1 r3` — așezi plăcuța la -1,1 cu rotirea 3.
3. `animal 0,1` — pui jetonul pe plăcuța de la 0,1.

## Reguli speciale și variante alese

Plăcuțele au 1 habitat (cheie de boltă: un singur animal) sau 2 habitate (2–3 animale). Un jeton pe o cheie de boltă dă un jeton-natură. Trei jetoane la fel în piață se schimbă automat.

## Punctaj

Urs: grupuri de exact 2 urși: 1 grup 5, 2 → 12, 3 → 20, 4+ → 30. Vidră: fiecare grup conex 1→1, 2→3, 3→6, 4→10, 5+→15. Zimbru: turmă de exact 3 = 9. Gaiță: gaițe fără altă gaiță alături: 1→2, 2→5, 3→9, 4→14, 5→20, 6→27, 7+→35. Vulpe: câte alte specii diferite are alături. Habitate: cel mai mare coridor al fiecărui habitat = atâtea puncte, +3 pentru cel mai mare (egalitate +1 fiecare). Fiecare jeton-natură rămas = 1 punct.

## Sfârșitul jocului

După 20 de ture pe jucător.

## Note despre AI și informația ascunsă

Informație deschisă; întâmplarea e doar la pachet și la sacul de jetoane; AI-ul nu citește piața (se reumple la întâmplare).

## Diferențe față de regulile tradiționale

Joc original; scorul este „regulile noastre”.
