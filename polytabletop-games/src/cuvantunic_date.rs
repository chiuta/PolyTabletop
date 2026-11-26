//! Cuvintele „Cuvântului Unic”: pentru fiecare, indicii de un singur cuvânt, de la cel
//! mai des gândit (primul) la cel mai rar (ultimul). Original.
//! Format: (cuvânt, „opt indicii separate prin spațiu”).

pub const WORDS: &[(&str, &str)] = &[
    (
        "pădure",
        "copaci ciuperci verde umbră ursuleț potecă frunze mușchi",
    ),
    (
        "ceas",
        "timp oră cadran bunic secunde pendulă alarma încheietură",
    ),
    (
        "pisică",
        "miau mustăți blană șoarece toarce coadă ghearele leneșă",
    ),
    (
        "ploaie",
        "umbrelă nori picături udă noroi vară burniță bălți",
    ),
    (
        "carte",
        "pagini citit bibliotecă autor poveste coperta raft litere",
    ),
    (
        "pâine",
        "brutărie făină cuptor coajă pâinică felie unt franzelă",
    ),
    ("telefon", "apel mobil ecran sună mesaj cablu agendă ureche"),
    ("avion", "zbor aripi aeroport cer pilot nori bilet decolare"),
    ("zăpadă", "iarnă alb frig fulgi sanie om schi nămete"),
    ("mare", "valuri sare plajă nisip apă vacanță corabie scoică"),
    ("munte", "pisc cabană vârf schi drumeție aer stâncă zăpadă"),
    (
        "inimă",
        "iubire puls roșu bătăi dragoste sânge piept sentiment",
    ),
    (
        "foc",
        "flacără cald fum lemn cenușă pompier scânteie chibrit",
    ),
    (
        "lună",
        "noapte stea cer luminează plină satelit lupi astronaut",
    ),
    ("soare", "lumină cald zi vară rază galben cer plajă"),
    ("vânt", "aer adiere furtună frunze zmeu suflă rece moară"),
    ("pește", "apă undiță înot solzi mare plasă acvariu pescar"),
    (
        "floare",
        "petale parfum primăvară buchet vază miros grădină albină",
    ),
    (
        "casă",
        "acasă acoperiș ușă familie locuință cămin geam curte",
    ),
    (
        "scaun",
        "masă șezut picioare birou lemn spătar așezat fotoliu",
    ),
    (
        "masă",
        "farfurie scaun mâncare prânz lemn față cină tacâmuri",
    ),
    (
        "pat",
        "somn pernă dormit noapte salteaua cearșaf vis dormitor",
    ),
    (
        "ușă",
        "clanță cheie intrare ușa încuiat deschisă lemn ferestre",
    ),
    (
        "fereastră",
        "geam perdea vedere sticlă cer deschisă ramă lumină",
    ),
    (
        "cheie",
        "lacăt ușă încuiat breloc deschide portar cifru inel",
    ),
    (
        "lampă",
        "lumină bec noapte abajur seară veioză aprinsă sticlă",
    ),
    (
        "oglindă",
        "reflexie chip rama sticlă baie privit machiaj spartă",
    ),
    (
        "cană",
        "ceai cafea toartă cald porțelan băutură masă ceașcă",
    ),
    (
        "cuțit",
        "lamă tăiat bucătărie ascuțit furculiță pâine tocător tacâm",
    ),
    (
        "lingură",
        "supă mâncare sorbit metal bucătărie tacâm linguriță fiertă",
    ),
    (
        "farfurie",
        "mâncare masă rotundă porțelan spălat sparte ciorbă ceramică",
    ),
    ("poartă", "gard curte intrare lemn închisă casă fotbal sat"),
    ("pod", "râu trecere apă peste arc fier metal drum"),
    (
        "drum",
        "mașină asfalt cale potecă lung orizont mers călătorie",
    ),
    (
        "mașină",
        "volan roți șofer benzină drum motor viteză claxon",
    ),
    (
        "bicicletă",
        "pedale roți ghidon lanț sport ciclist cască parc",
    ),
    (
        "tren",
        "gară șine vagon călătorie peron locomotivă bilet clopot",
    ),
    ("barcă", "apă vâsle lac pescar valuri velă mal plutește"),
    (
        "insulă",
        "mare nisip palmier pustie naufragiu tropical comoară apă",
    ),
    ("deșert", "nisip cămilă sete cald cactus oază dună soare"),
    (
        "junglă",
        "liane maimuță tropical verde ploaie animale umed Amazon",
    ),
    (
        "vulcan",
        "lavă erupție munte cenușă crater foc fierbinte cenușă",
    ),
    (
        "cutremur",
        "tremur scuturat clădiri seism panică ruine Vrancea magnitudine",
    ),
    ("curcubeu", "culori ploaie soare șapte cer arc comoară nor"),
    (
        "furtună",
        "tunet fulger vânt nori ploaie întuneric ceață trăsnet",
    ),
    (
        "fulger",
        "tunet cer lumină furtună trăsnet strălucire nor zig",
    ),
    (
        "stea",
        "cer noapte lumină cosmos strălucire cade dorință galaxie",
    ),
    (
        "planetă",
        "Pământ cosmos orbită Marte sistem rotund inel soare",
    ),
    (
        "rachetă",
        "spațiu cosmos lansare astronaut foc zbor Lună viteză",
    ),
    (
        "robot",
        "metal viitor mașină cip automat om cablu inteligență",
    ),
    (
        "computer",
        "tastatură ecran mouse internet program birou joc laptop",
    ),
    (
        "internet",
        "rețea site căutare wifi online navigare pagini lume",
    ),
    (
        "jucărie",
        "copil joacă păpușă cutie mașinuță magazin plastic cadou",
    ),
    (
        "păpușă",
        "fetiță joacă rochie jucărie păr plastic ochi casă",
    ),
    ("minge", "fotbal rotund joc sărit teren aruncat gol sport"),
    (
        "balon",
        "aer zbor aniversare sărbătoare colorat umflat petrecere sfoară",
    ),
    (
        "tort",
        "aniversare lumânări dulce zi felie crema cadouri petrecere",
    ),
    (
        "ciocolată",
        "dulce cacao negru tabletă lapte pofta maro bomboane",
    ),
    (
        "înghețată",
        "vară dulce rece cornet vanilie gelato căldură lins",
    ),
    (
        "cafea",
        "dimineață cană cofeină espresso cald boabe aromă trezește",
    ),
    (
        "ceai",
        "cană cald plante lămâie abur ceainic fierbinte frunze",
    ),
    ("lapte", "vacă alb copil cereale băutură iaurt cană biberon"),
    (
        "brânză",
        "lapte vacă gaură șoarece telemea cașcaval mucegai aluat",
    ),
    ("măr", "fruct roșu copac Eva compot pom gutui verde"),
    (
        "banană",
        "fruct maimuță galbenă coajă tropical dulce lungă smoothie",
    ),
    ("lămâie", "acru galbenă suc ceai citrice miez zeamă felie"),
    ("ou", "găină cojă gălbenuș Paște clocit omletă fiert cuib"),
    (
        "pizza",
        "italiană brânză felie cuptor roșii livrare rotundă aluat",
    ),
    (
        "supă",
        "ciorbă lingură cald oală fiartă zarzavat farfurie aburi",
    ),
    ("grătar", "carne mici cărbuni vară fum friptură curte bere"),
    (
        "sare",
        "mare condiment bucătărie sărat piper mâncare cristale masă",
    ),
    (
        "zahăr",
        "dulce ceai cub alb cafea bomboană cristale dulceață",
    ),
    ("miere", "albine dulce stup aur borcan ceai lipicios fagure"),
    ("albină", "miere stup zumzet polen floare ac roi regină"),
    (
        "fluture",
        "aripi colorat insectă omidă floare zbor primăvară ușor",
    ),
    (
        "furnică",
        "mică lucrătoare colonie zahăr furnicar harnică mușuroi picior",
    ),
    (
        "păianjen",
        "pânză opt picioare insectă țese muscă frică colț",
    ),
    ("șarpe", "veninos frică limbă reptilă târâș piele cobră gaz"),
    (
        "broască",
        "salt lac verde coaxă prinț baltă mlaștină țestoasă",
    ),
    (
        "cal",
        "galop călăreț cămașă iapă grajd armăsar trap potcoavă",
    ),
    ("vacă", "lapte pășune muu ferma iarbă coarne stână mugit"),
    ("oaie", "lână turmă cioban baa munte miel pășune stână"),
    (
        "porc",
        "grunz noroi Crăciun slănină fermă gospodărie cârnat mistreț",
    ),
    ("găină", "ou cloșcă cocoș cuib cotcodac puiet fermă pene"),
    ("rață", "lac apă cuac pene boboc cioc gâscă pui"),
    ("lup", "urlet haită pădure Scufița lună flămând sălbatic oi"),
    (
        "urs",
        "miere hibernare ghiare Carpați pădure labă blană sălbatic",
    ),
    (
        "elefant",
        "trompă colți mare Africa urechi gri memorie circ",
    ),
    (
        "leu",
        "rege coamă Africa răget leoaică savană ghearele gheara",
    ),
    ("maimuță", "banană copac junglă coadă salt zoo agilă imită"),
    (
        "pinguin",
        "Antarctica gheață alb negru frig pește gheață pui",
    ),
    (
        "delfin",
        "mare sărit inteligent apă pește mamifer spectacol zâmbet",
    ),
    ("rechin", "dinți mare pește pericol aripă apă înot film"),
    (
        "dinozaur",
        "fosilă uriaș preistorie ou muzeu extincție dinte jur",
    ),
    ("dragon", "foc aripi comoară cavaler basm zmeu solzi castel"),
    (
        "vrăjitoare",
        "mătură cazan poțiune pălărie magie basm vrajă pisică",
    ),
    (
        "prinț",
        "castel prințesă basm calare coroană rege cal poveste",
    ),
    (
        "regină",
        "coroană castel rege tron albină palat sceptru șah",
    ),
    (
        "fantomă",
        "alb noapte sperietură castel lanț cearșaf frică Halloween",
    ),
    (
        "pirat",
        "corabie comoară ochi sabie papagal steag cufăr insulă",
    ),
    ("cowboy", "cal pălărie vest lasou pistol ranch vaci prerie"),
    ("clovn", "circ nas roșu amuzant mască râs peruca spectacol"),
    (
        "circ",
        "clovn cort trapez animale spectacol leu bilet acrobat",
    ),
    (
        "teatru",
        "scenă actori cortină piesă loje spectacol aplauze sală",
    ),
    (
        "film",
        "cinema ecran actori regizor bilet popcorn scenariu seară",
    ),
    (
        "muzică",
        "cântec note ritm melodie radio concert chitară ureche",
    ),
    (
        "chitară",
        "corzi cântec acord muzică lemn rock trupă serenadă",
    ),
    ("pian", "clape muzică concert negru alb note pedală sală"),
    (
        "tobă",
        "bețe ritm zgomot muzică bătaie percuție trupă bubuit",
    ),
    (
        "dans",
        "vals mișcare muzică pas balet petrecere pereche ritm",
    ),
    (
        "pictură",
        "pânză pensulă culori muzeu tablou artist șevalet vopsea",
    ),
    (
        "statuie",
        "piatră marmură muzeu sculptor oraș bronz piață erou",
    ),
    (
        "muzeu",
        "exponate istorie vitrină bilet artă tablouri ghid liniște",
    ),
    (
        "bibliotecă",
        "cărți raft liniște citit împrumut bibliotecar studiu pagini",
    ),
    (
        "școală",
        "elevi clasă profesor lecție caiet clopoțel temă ghiozdan",
    ),
    (
        "profesor",
        "școală clasă tablă notă lecție examen elevi cretă",
    ),
    (
        "examen",
        "stres notă școală studiu test bilet facultate teza",
    ),
    (
        "doctor",
        "spital halat boală stetoscop pacient consult medicament alb",
    ),
    (
        "spital",
        "doctor pacient pat bolnav alb urgență salvare asistentă",
    ),
    (
        "medicament",
        "pastilă farmacie boală doctor rețetă sirop tratament doză",
    ),
    (
        "pompier",
        "foc furtun sirenă curaj scară incendiu mașină roșie",
    ),
    (
        "poliție",
        "mașină sirenă ordine amendă hoț lege kepi sector",
    ),
    ("hoț", "furt noapte mască poliție închisoare sac bani fugă"),
    (
        "bani",
        "monedă bancnotă portofel bancă avere preț plată salariu",
    ),
    ("bancă", "bani cont credit seif card ghișeu depozit dobândă"),
    (
        "magazin",
        "cumpărături vitrină preț casă client raft marfă ofertă",
    ),
    (
        "piață",
        "legume târg fructe tarabă vânzător sac mulțime dimineață",
    ),
    (
        "oraș",
        "străzi blocuri mașini lume metrou lumini clădiri centru",
    ),
    ("sat", "casă fântână uliță țară gard bătrâni livadă clopot"),
    ("fermă", "vaci porci găini tractor câmp fân hambar țăran"),
    (
        "grădină",
        "flori legume stropitoare plante iarbă gard buruieni sol",
    ),
    (
        "copac",
        "frunze trunchi umbră pădure rădăcină cuib crengi pădure",
    ),
    (
        "frunză",
        "verde copac toamnă cade ramură vânt ierbar uscată",
    ),
    ("iarbă", "verde pajiște tuns gazon fir pășune ierburi câmp"),
    ("nor", "cer ploaie alb pufos vânt umbră ceață fum"),
    ("râu", "apă curge pod mal pește izvor malul vale"),
    ("lac", "apă pește barcă luciu liniște mal lebădă vară"),
    ("izvor", "apă munte rece râu cristalin fântână pur sursă"),
    (
        "cascadă",
        "apă cădere stâncă munte spumă curcubeu zgomot picătură",
    ),
    (
        "peșteră",
        "întuneric stalactite liliac adânc piatră ecou ursuleț ascuns",
    ),
    (
        "comoară",
        "aur hartă cufăr pirat monede ascuns bogăție insulă",
    ),
    (
        "hartă",
        "drum țări busolă desen orientare comoară atlas munți",
    ),
    (
        "busolă",
        "nord orientare ac drum cardinal excursie navigație ghid",
    ),
    (
        "cort",
        "camping noapte pădure somn excursie aer liber țăruși",
    ),
    (
        "rucsac",
        "spate excursie drumeție bretele călătorie ghiozdan geamantan haine",
    ),
    (
        "valiză",
        "călătorie bagaj aeroport vacanță roți haine plecare cheie",
    ),
    (
        "hotel",
        "cameră vacanță recepție dormit noapte turist lux cheie",
    ),
    ("plajă", "nisip mare vară soare umbrelă prosop valuri bronz"),
    (
        "pașaport",
        "viză graniță călătorie vamă fotografie țară ștampilă act",
    ),
    ("steag", "țară culori pânză vânt simbol tricolor catarg pol"),
    (
        "coroană",
        "rege aur cap regină bijuterie tron diamant castel",
    ),
    ("castel", "turn rege cavaler zid piatră prințesă pod ruină"),
    ("turn", "înalt castel clopot vârf scară ceas piatră Pisa"),
    (
        "biserică",
        "clopot rugăciune cruce lumânare slujbă turn preot icoană",
    ),
    (
        "clopot",
        "sunet biserică turn școală bronz clopoțel sună limbă",
    ),
    (
        "lumânare",
        "flacără ceară ceară aprinsă tort biserică întuneric fitil",
    ),
    (
        "cadou",
        "ziua surpriză pachet panglică Crăciun ambalaj dar cutie",
    ),
    (
        "Crăciun",
        "brad cadouri Moș iarnă colinde sărbătoare cozonac zăpadă",
    ),
    (
        "Paște",
        "ouă cozonac înviere iepuraș sărbătoare primăvară biserică miel",
    ),
    (
        "nuntă",
        "mireasă mire tort dans inele petrecere biserică lăutari",
    ),
    (
        "inel",
        "deget aur nuntă logodnă bijuterie diamant cerc rotund",
    ),
    (
        "bijuterie",
        "aur diamant colier inel scump piatră argint lanț",
    ),
    (
        "diamant",
        "scump piatră inel strălucitor logodnă dur carat bijuterie",
    ),
    (
        "aur",
        "galben scump bijuterie comoară metal lingou monedă strălucitor",
    ),
    (
        "argint",
        "metal bijuterii lingură lună gri scump aliaj monedă",
    ),
    (
        "sticlă",
        "geam spartă transparentă pahar reciclare fragil borcan apă",
    ),
    (
        "hârtie",
        "scris foaie caiet alb carte imprimantă pliat copac",
    ),
    (
        "creion",
        "scris desen ascuțit mină radieră școală grafit lemn",
    ),
    (
        "pix",
        "scris cerneală albastru semnătură caiet birou clic mine",
    ),
    (
        "scrisoare",
        "plic timbru poștaș adresă mesaj hârtie dor poștă",
    ),
    (
        "poștă",
        "scrisoare timbru colet poștaș ghișeu cutie adresă plic",
    ),
    (
        "camera",
        "foto aparat poză lentilă bliț film amintiri flash",
    ),
    (
        "televizor",
        "ecran telecomandă canal film seară știri reclamă antenă",
    ),
    (
        "radio",
        "muzică unde antenă știri emisiune frecvență difuzor post",
    ),
    ("bec", "lumină idee electric lampă fir aprins soclu sticlă"),
    (
        "baterie",
        "curent energie telefon încărcat plus minus uzat putere",
    ),
    ("fir", "cablu ață sârmă lungime subțire cusut ac aței"),
    ("ac", "ață cusut ascuțit înțepătură seringă pin fân croitor"),
    (
        "foarfecă",
        "tăiat hârtie lame croitor frizer două foi ascuțit",
    ),
    (
        "ciocan",
        "cuie lemn unealtă bătut lovit tâmplar fier ciocănit",
    ),
    (
        "scară",
        "trepte urcat înălțime perete pompier etaj bloc lemn",
    ),
    ("lift", "etaj buton bloc urcare scară cabină ușă sus"),
    (
        "umbrelă",
        "ploaie nori mâner picături protecție colorat vânt deschisă",
    ),
    (
        "pălărie",
        "cap soare boruri joben cowboy panglică vară haine",
    ),
    (
        "pantofi",
        "picioare tocuri șireturi perechi talpă încălțăminte piele magazin",
    ),
    (
        "ochelari",
        "vedere lentile dioptrii nas ramă soare citit miop",
    ),
    ("haină", "iarnă frig ghardă fermoar cuier cald ploaie piele"),
    (
        "cravată",
        "gât costum nod birou eleganță dungi bărbat serviciu",
    ),
    (
        "rochie",
        "fustă mireasă eleganță femeie seară croitor mătase dansul",
    ),
    (
        "blugi",
        "pantaloni albastru denim buzunare uzat modă genunchi curea",
    ),
    (
        "pieptene",
        "păr dinți frizer coafură piaptăn baie ac ordonat",
    ),
    ("săpun", "baie spumă curat mâini miros apă spălat parfum"),
    ("apă", "sete râu mare ploaie robinet curat pahar vital"),
    ("gheață", "rece cuburi iarnă patinaj topit îngheț polar cub"),
    ("fum", "foc țigară coș negru ceață miros grătar nor"),
    (
        "umbră",
        "soare copac întuneric siluetă lumină negru răcoare secret",
    ),
    (
        "vis",
        "somn noapte coșmar dorință pat imaginar adormit realitate",
    ),
    (
        "secret",
        "taină ascuns tăcere șoaptă cheie confidențial prieten sigiliu",
    ),
    (
        "prieten",
        "amic tovarăș drag încredere ajutor joacă prietenie vechi",
    ),
    ("familie", "mamă tată copii casă bunici frați acasă masă"),
    (
        "bunică",
        "poveste cozonac iubire nepoți casă cămin bătrână tricotat",
    ),
    ("copil", "joacă părinți școală mic râs jucărie plâns viitor"),
    ("bebeluș", "biberon scutec plâns mamă somn mic leagăn pătuț"),
    (
        "eroină",
        "curajoasă poveste cărți salvare film erou eroină eroică",
    ),
    (
        "detectiv",
        "indiciu lupă crimă mister caz urme suspect pipă",
    ),
    (
        "magie",
        "vrajă baghetă iluzie truc scamator vrăjitor poțiune carte",
    ),
    ("poveste", "basm carte seară somn erou citit început bunică"),
    (
        "glumă",
        "râs umor amuzant poantă anecdotă haz clovn bancuri",
    ),
    (
        "liniște",
        "tăcere pace zgomot bibliotecă somn calm șoaptă noapte",
    ),
    (
        "pace",
        "război liniște porumbel armonie tratat calm olivier lume",
    ),
    (
        "libertate",
        "zbor pasăre păsări eliberare drept lanț cer drumul",
    ),
    (
        "timp",
        "ceas trecut viitor oră clipă calendar prezent secunde",
    ),
    (
        "noapte",
        "întuneric lună stele somn lampă vis liniște negru",
    ),
    (
        "dimineață",
        "răsărit cafea cocoș trezit soare rouă proaspăt zori",
    ),
    (
        "culoare",
        "roșu pictură curcubeu pensulă vopsea nuanță paletă galben",
    ),
    (
        "zâmbet",
        "bucurie dinți față râs fericire oglindă amabil buze",
    ),
    (
        "lacrimă",
        "plâns tristețe ochi picătură obraz durere cădere sare",
    ),
    (
        "durere",
        "rană doare suferință spital pastilă tratament inimă amar",
    ),
    (
        "curaj",
        "frică erou viteaz pompier leu îndrăzneală faptă eroism",
    ),
    (
        "frică",
        "teamă întuneric fantomă tremur șarpe groază panică speriat",
    ),
    (
        "energie",
        "curent soare putere baterie electric forță vlagă izvor",
    ),
    (
        "viteză",
        "mașină alergare grabă curse repede kilometri fulger mișcare",
    ),
    (
        "greutate",
        "kilograme cântar mare povară haltere masa mult greu",
    ),
    (
        "echilibru",
        "balanță funie corp cădere cântar stabil bicicletă stare",
    ),
];
