/**
 * @param {string} s
 * @return {number}
 */
const romanToInt = function(s) {
    let r2i = {
        I: 1,
        V: 5,
        X: 10,
        L: 50,
        C: 100,
        D: 500,
        M: 1000
    }

    let result = 0;

    for (let i = 0; i < s.length; i++) {
        if (r2i[s[i]] < r2i[s[i+1]]) {
            result -= r2i[s[i]];
        } else {
            result += r2i[s[i]];
        }
    }

    return result;
};

console.log(romanToInt("III"));
console.log(romanToInt("LVIII"));
console.log(romanToInt("MCMXCIV"));