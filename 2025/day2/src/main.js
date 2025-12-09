var fs = require("fs");
var path = require("path");
function main() {
    var answer = 0;
    var inputPath = path.join(__dirname, "..", "input");
    var fileContents = fs.readFileSync(inputPath, "utf8");
    var input = fileContents
        .split(",")
        .map(function (s) { return s.trim(); })
        .filter(Boolean);
    for (var i = 0; i < input.length; i++) {
        var _a = input[i].split("-"), first = _a[0], last = _a[1];
        var firstNum = Number(first);
        var lastNum = Number(last);
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
function isInvalid(num) {
    var numStr = num.toString();
    var numLen = numStr.length;
    if (numLen <= 1)
        return false;
    for (var chunkSize = 1; chunkSize <= numLen / 2; chunkSize++) {
        if (numLen % chunkSize !== 0)
            continue;
        var chunk = numStr.slice(0, chunkSize);
        if (numStr === chunk.repeat(numLen / chunkSize))
            return true;
    }
    return false;
}
function isInvalidRegex(num) {
    var numStr = num.toString();
    return /^(.+?)\1+$/.test(numStr);
}
var x = main();
console.log("solution:".concat(x));
