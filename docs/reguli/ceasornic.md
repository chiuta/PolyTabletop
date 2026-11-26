# Ceasornicul din Valea Ceții (deducție cu povestitor, masa = roboți) (`ceasornic`)

Ceasornicul din Valea Ceții: deducție socială cu povestitor automat, 12 locuri (tu + 11 roboți), variantă originală.

## Scop

Binele: omoară Strigoiul (fără moștenitor). Răul: rămân doi vii, Sfântul e executat sau trec 20 de zile.

## Pregătire

Scenariul are 14 roluri: 9 localnici (șapte în joc), 2 străini (Bețivul și Sfântul), 2 slugi (Otrăvitorul și Moștenitoarea) și Strigoiul. Localnicii și străinii formează echipa bună; slugile și Strigoiul, echipa rea. Povestitorul e automat (masa). Bețivul crede că e un localnic care nu e în joc și primește mereu informații false; un jucător „otrăvit” primește informații false până seara următoare.

## Desfășurarea unei ture

Noaptea: rolurile cu informație (Ghicitoarea, Cronicarul, Veghea, Cercetașul, Groparul) află ceva, Călugărul ocrotește, Strigoiul ucide din noaptea 2. Ziua roboții își spun rolul (`cer N` îi întreabă), tu declari adevărul (`declar`) sau minți (`mint rol`), nominalizezi (`nominez 6`) și votezi (`da`/`nu`); cine strânge jumătate din vii și cel mai mult e executat la apus.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `cer 4` — ceri locului 4 să-și spună rolul
* `declar` — spui adevărul despre rolul tău
* `mint groparul` — pretinzi că ești Groparul
* `nominez 6` — nominalizezi locul 6
* `da / nu` — votezi
* `treci / pas` — treci
* `ghicesc 3 7 / otravesc 3 / ocrotesc 3 / ucid 3 / vanez 5` — acțiuni de noapte, după rol
* `polytabletop` — povestitorul / roboții acționează (forțat)

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play ceasornic --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `cer 8` — îl întrebi pe locul 8 ce rol are (răspunde Cercetașul).
2. `cer 4` — îl întrebi pe locul 4 (răspunde Ghicitoarea).
3. `mint groparul` — pretinzi că ești Groparul (ești Moștenitoarea — o slugă a răului).

## Reguli speciale și variante alese

Cei morți au un singur vot „fantomă” pe joc. Roboții pot mărturisi adevărul sau minți.

## Punctaj

Nu există scor; contează echipa învingătoare.

## Sfârșitul jocului

Binele câștigă când Strigoiul moare; răul când rămân doi vii, când Sfântul e executat sau când trec 20 de zile.

## Note despre AI și informația ascunsă

Roboții buni folosesc doar informația lor privată și afirmațiile publice; roboții răi își cunosc echipa. Rolul tău e afișat doar ție.

## Diferențe față de regulile tradiționale

Joc original; toate numele, rolurile și textele sunt originale.
