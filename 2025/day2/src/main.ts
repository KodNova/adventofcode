const fs = require("fs");
const path = require("path");

function main(): number {
  let answer: number = 0;
  const inputPath = path.join(__dirname, "..", "input");
  const fileContents = fs.readFileSync(inputPath, "utf8");

  const input: string[] = fileContents
    .split(",")
    .map((s: string) => s.trim())
    .filter(Boolean);

  for (let i = 0; i < input.length; i++) {
    const [first, last] = input[i].split("-");

    let firstNum: number = Number(first);
    let lastNum: number = Number(last);

    while (firstNum <= lastNum) {
      if (isInvalid(firstNum)) {
        answer += firstNum;
        console.log(firstNum);
      }

      firstNum++;
    }
  }

  return answer;
}

function isInvalid(num: number): boolean {
  const numStr: string = num.toString();
  const numLen: number = numStr.length;

  if (numLen <= 1) return false;

  for (let chunkSize = 1; chunkSize <= numLen / 2; chunkSize++) {
    if (numLen % chunkSize !== 0) continue;

    const chunk: string = numStr.slice(0, chunkSize);

    if (numStr === chunk.repeat(numLen / chunkSize)) return true;
  }

  return false;
}

function isInvalidRegex(num: number): boolean {
  const numStr = num.toString();
  return /^(.+?)\1+$/.test(numStr);
}

const x = main();

console.log(`solution:${x}`);
