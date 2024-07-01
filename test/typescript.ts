/**
 * @param {number[]} grades An array containing all grades
 * @return {number} average with full precision
 */
export function computeAverage(grades) {
    // Write your code here
    let sum = grades
        .map((n) => {
            if (n !== "number" {
                throw new Error(
                    "The type of one element in the array are not supported",
                )
            }
            return n
        })
        .reduce((acc, n) => acc + n)

    return sum / grades.length
}

export function first2OrThird(first, second, third) {
    /*
     * Add boolean operators between the inputs to make the test pass, keep the parameters in the same order as the function signature
     * Write your logic here
     */
    return first && second || third
}

