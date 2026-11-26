# Mangala (mancala turcească) (`mangala`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play mangala --seed 1`. Sursa implementării: `polytabletop-games/src/mangala.rs`.

> Rezumat din `polytabletop list`: 2 × 6 gropi, 4 semințe, câte un depozit. Prima sămânță rămâne în groapa de start. Ultima în depozit: mai muți; ultima într-o groapă adversă ajunsă la număr par: o iei; ultima într-o groapă proprie goală: iei groapa opusă. Câștigă depozitul mai mare. Mutări: 1–6.

## Scop

Ai depozitul mai plin la sfârșit.

## Pregătire

2 × 6 gropi, 4 semințe în fiecare; un depozit pentru fiecare, după ultima groapă proprie.

## Desfășurarea turei

Alegi o groapă proprie nevidă. Dacă are mai mult de o sămânță, prima rămâne în aceeași groapă, apoi se seamănă trigonometric (prin depozitul propriu, peste cel al adversarului); dacă are una, trece în groapa următoare.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Numărul gropii proprii: `1`–`6`.

## Reguli speciale și variante alese

* Ultima sămânță în depozitul propriu: mai muți o dată.
* Ultima sămânță într-o groapă a adversarului care ajunge la număr par: iei toată groapa.
* Ultima sămânță într-o groapă proprie goală: iei sămânța și groapa opusă (dacă are semințe).

## Punctaj

Semințele din depozit.

## Sfârșitul jocului

Când un jucător nu mai are semințe în gropi, celălalt își pune restul în depozit; câștigă depozitul mai mare (24–24 = remiză). După 600 de mutări fiecare își ia semințele de pe partea proprie.

## Note despre AI

Informație completă. Alpha-beta cu adâncime fixă 7.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar nu are informație ascunsă.

## Diferențe față de regulile tradiționale

* Regula capturii în groapa proprie goală diferă între descrieri; s-a ales varianta „iei groapa opusă”.
