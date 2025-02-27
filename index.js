const { execSync } = require("child_process");
const { performance } = require("perf_hooks");

const startTime = performance.now(); // Start time

const output = execSync(`./target/release/front-panel`, { encoding: "utf8" });

const endTime = performance.now(); // End time

console.log(`Execution Time: ${(endTime - startTime).toFixed(3)} ms`);
console.log(output);