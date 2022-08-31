// Deno.core.print("Hello runjs");
console.log("Hello", "runjs!");
console.error("Boom!");

const path = "./log.txt";

try {
    const contents = await runjs.readFile(path);
    console.log("Read from a file", contents);
} catch (err) {
    console.error("Unable to read file", path, err);
}

await runJs.writeFile(path, "I can write to a file.");
const contents = await runJs.readFile(path);

console.log("Read from a file", path, "Contents: ", contents);
console.log("Removing file", path);
runJs.removeFile(path);

console.log("File removed");