// https://adventjs.dev/en/challenges/2024/11

/**
 * @param {string} filename - The filename to decode.
 * @returns {string} The decoded filename.
 */
function decodeFilename(filename) {
    return filename.match(/^\d+_([^.]+\.[^.]+)\.[^.]+$/)[1];
}
