# Cuvânt Unic (indicii care se anulează) (`cuvantunic`)

Cuvânt Unic: joc cu indicii dintr-un singur cuvânt, în duel pe 12 runde (224 de cuvinte cu lexicon de indicii).

## Scop

Ai mai multe puncte după 12 runde.

## Pregătire

Într-o rundă un jucător ghicește (alternativ J1/J2), celălalt știe cuvântul secret și alege un indiciu din lexiconul lui. Doi „martori” robotici aleg și ei fiecare un indiciu.

## Desfășurarea unei ture

Cel care dă indiciul: `indiciu N`. Pasul forțat `dezvaluie`: martorii aleg indicii (mai des pe cele obișnuite); indiciile identice se anulează între ele. Ghicitorul vede ce a rămas și alege unul dintre cei 6 candidați (`ghici 4`) sau `pas`. Pasul forțat `verifica` decontează.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `indiciu 1` — dai indiciul 1 din lexicon
* `ghici 4` — ghicești candidatul 4
* `pas` — nu ghicești
* `dezvaluie` — pas forțat
* `verifica` — pas forțat

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play cuvantunic --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `ghici 4` — ghicești candidatul 4.
2. `indiciu 1` — în runda următoare dai indiciul 1 din lexicon.
3. `ghici 6` — ghicești candidatul 6.

## Reguli speciale și variante alese

Indiciile obișnuite se anulează mai ușor.

## Punctaj

Ghicit: +1 ghicitorului; greșit: +1 pentru cel care a dat indiciul; pas: nimic.

## Sfârșitul jocului

Cine are mai multe puncte după 12 runde câștigă.

## Note despre AI și informația ascunsă

AI-ul care dă indiciul alege indicii obișnuite (se anulează mai des); cel care ghicește potrivește indiciile rămase cu lexiconul fiecărui candidat. Niciunul nu vede răspunsul în momentul alegerii.

## Diferențe față de regulile tradiționale

Adaptare pentru doi jucători și duel pe 12 runde a mecanicii genului.
