# Mahjong (Hong Kong, 2 jucători) (`mahjong`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play mahjong --seed 1`. Sursa implementării: `polytabletop-games/src/mahjong.rs`.

> Rezumat din `polytabletop list`: 136 de piese, 13 în mână, zid viu de 96 și zid mort de 14 (ascuns; vezi doar câte au rămas). Tragi și arunci; adversarul poate lua aruncarea cu pung / kong / chi (de la oricare aruncare) sau pe câștig. Mână câștigătoare = 4 seturi + pereche și minimum 1 fan (tragere proprie, mână închisă, toate secvențele, toate pungurile, o culoare, dragoni, vânturi). Fără șapte perechi / mâini speciale. Zid epuizat = remiză. Mutări: trage, d5m, d Dr, mahjong, pung, kong, chi2m3m, kong5m, pas, trage-moarta.

## Scop

Declari primul o mână câștigătoare: 4 seturi (secvențe, pungs sau careuri) și o pereche, cu cel puțin 1 fan.

## Pregătire

136 de piese: 3 culori numerice × 9 × 4 (caractere `m`, bambus `s`, cercuri `p`), 4 vânturi (E, S, W, N) și 3 dragoni (Dr roșu, Dg verde, Dw alb); fără flori. Fiecare primește 13 piese; J1 e dealerul (vântul Est), J2 vântul Sud; vântul rundei e Est. 14 piese sunt rezervate ca zid mort; cele 96 rămase se trag pe rând. Zidul este ascuns: vezi doar câte piese au rămas.

## Desfășurarea turei

Tragi (`trage`), apoi arunci (`d5m`) sau declari câștigul. Adversarul poate lua piesa aruncată. După pung sau chi arunci fără să tragi. Dacă nu ai nicio reacție posibilă, tragi direct.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `trage`; aruncare: `d5m`, `d Dr` (d + piesa).
* Reacții la aruncare: `mahjong`, `pung`, `kong`, `chi2m3m` (secvență cu două piese proprii; cu doi jucători e permis de la orice aruncare), `pas`.
* Careu din mână sau adăugat la un pung expus: `kong5m`; urmează `trage-moarta` (tragerea din zidul mort).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (1): `trage`. Introduci `trage`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (14): `d3m`, `d4m`, `d6m`, `d7m`, `d2s`, `d4s`, `d6s`, `d1p`, `d4p`, `d5p` …. Introduci `d1p`; răspunsul calculatorului (în ordine): `trage`; `d1m`.
3. Mutări legale (1): `trage`. Introduci `trage`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Fani: tragere proprie 1, mână închisă 1, toate secvențele 1, toate pungurile 3, o culoare + onoruri 3, o culoare pură 6, doar onoruri 10, pung de dragon 1, pung de vânt propriu 1, pung de vântul rundei 1.
* Nu există șapte perechi, treisprezece orfani și alte mâini speciale.

## Punctaj

Fanii se afișează la final; câștigă primul care declară o mână validă.

## Sfârșitul jocului

La mână validă declarată (victorie) sau când zidul viu se epuizează fără câștigător (remiză).

## Note despre AI

Informație ascunsă: zidul, zidul mort și mâna adversarului. AI-ul vede doar mâna lui, seturile expuse și aruncările publice și alege după un scor de potențial al mâinii.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Variantă Hong Kong simplificată pentru 2 jucători, fără flori, fără mâini speciale, punctaj redus la verificarea celor 4 seturi + pereche și a minimului de 1 fan.
