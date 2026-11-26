// Test: logica de indexare și swipe din tools/site/lightbox.js (între marcajele LOGIC). Rulare: node tools/site/test_lightbox.js
const fs = require("fs"), path = require("path"), assert = require("assert");
const src = fs.readFileSync(path.join(__dirname, "lightbox.js"), "utf8");
const logic = src.slice(src.indexOf("// LOGIC"), src.indexOf("// /LOGIC"));
const { lbStep, lbSwipe } = new Function(logic + "\nreturn {lbStep,lbSwipe};")();
assert.strictEqual(lbStep(0, 6, "ArrowRight"), 1);
assert.strictEqual(lbStep(5, 6, "ArrowRight"), 0);   // înfășurare
assert.strictEqual(lbStep(0, 6, "ArrowLeft"), 5);
assert.strictEqual(lbStep(2, 6, "PageDown"), 3);
assert.strictEqual(lbStep(2, 6, "PageUp"), 1);
assert.strictEqual(lbStep(3, 6, "Home"), 0);
assert.strictEqual(lbStep(3, 6, "End"), 5);
assert.strictEqual(lbStep(3, 6, "x"), 3);
assert.strictEqual(lbStep(0, 1, "next"), 0);
assert.strictEqual(lbStep(0, 0, "next"), 0);
assert.strictEqual(lbSwipe(-80, 5), "next");
assert.strictEqual(lbSwipe(80, 5), "prev");
assert.strictEqual(lbSwipe(30, 0), "");               // prea scurt
assert.strictEqual(lbSwipe(60, 90), "");              // gest vertical
console.log("test_lightbox: OK (node " + process.version + ")");
