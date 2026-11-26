// Test: obiectul T din docs/index.html și din paginile de joc are aceleași chei, nevide, în cele 8 limbi.
// Rulare: node tools/site/test_i18n.js   (testat cu Node 22; nu necesită pachete)
const fs = require("fs"), path = require("path");
const docs = path.join(__dirname, "..", "..", "docs");
const LANGS = ["en", "ro", "fr", "it", "es", "pt", "de", "hu"];
let bad = 0;
function load(file) {
  const t = fs.readFileSync(file, "utf8");
  const i = t.indexOf("const T=") + 8;
  let j = t.indexOf(";\nconst D=", i);
  if (j < 0) j = t.indexOf("\n};\n", i) + 2;
  const m = [null, t.slice(i, j)];
  const TX = (t.match(/const TX=(\{.*\});\n/) || [])[1];
  const T = new Function("return " + m[1])();
  if (TX) { const x = JSON.parse(TX); for (const l of LANGS) Object.assign(T[l], x[l]); }
  return { t, T };
}
function check(file) {
  const { t, T } = load(file);
  const keys = new Set([...t.matchAll(/data-(?:i18n|i18n-aria|i18n-ph|i|ia|ialt)="(\w+)"/g)].map(m => m[1]));
  const ref = Object.keys(T.en);
  for (const l of LANGS) {
    if (!T[l]) { console.log(file, "limba lipsă", l); bad++; continue; }
    for (const k of keys) if (!T[l][k]) { console.log(path.basename(file), "cheie lipsă", l, k); bad++; }
    for (const k of ref) if (!(k in T[l]) || T[l][k] === "") { console.log(path.basename(file), "gol/lipsă", l, k); bad++; }
    // substituții {x}: aceleași în toate limbile
    for (const k of ref) {
      const a = (T.en[k].match(/\{\w+\}/g) || []).sort().join(), b = ((T[l][k] || "").match(/\{\w+\}/g) || []).sort().join();
      if (a !== b) { console.log(path.basename(file), "substituții diferite", l, k, a, b); bad++; }
    }
  }
  return ref.length;
}
const files = [path.join(docs, "index.html"), path.join(docs, "jocuri", "index.html"), ...fs.readdirSync(path.join(docs, "jocuri")).filter(f => f.endsWith(".html")).map(f => path.join(docs, "jocuri", f))];
const n = {};
for (const f of files) n[path.relative(docs, f)] = check(f);
console.log("fișiere verificate:", files.length, "| chei în index.html:", n["index.html"] + "/" + n[path.join("jocuri","index.html")], "| node", process.version, "| probleme:", bad);
process.exit(bad ? 1 : 0);
