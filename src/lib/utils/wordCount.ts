/** Count whitespace-delimited words while treating each Han character as a word. */
export function countWordsInText(text: string): number {
  return text
    .trim()
    .split(/\s+/u)
    .filter(Boolean)
    .reduce((count, token) => {
      const han = [...token].filter((character) => /\p{Script=Han}/u.test(character)).length;
      const nonHanWord = [...token].some(
        (character) => !/\p{Script=Han}/u.test(character) && /[\p{L}\p{N}_]/u.test(character)
      );
      return count + han + (han === 0 || nonHanWord ? 1 : 0);
    }, 0);
}

/** Match saved prose counting: block boundaries separate words; inline marks do not. */
export function countWordsInHtml(html: string | null | undefined): number {
  if (!html) return 0;
  const document = new DOMParser().parseFromString(html, "text/html");
  for (const node of document.querySelectorAll("p, div, br, hr, li, h1, h2, h3, blockquote")) {
    node.before(" ");
    node.after(" ");
  }
  const text = document.body.textContent?.trim() ?? "";
  return countWordsInText(text);
}
