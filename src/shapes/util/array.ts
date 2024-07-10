/** integer array from start to end-1 with steps of 1
 * ```js
 * range(0,5) // [0,1,2,3,4]
 * ```*/
export const range = (start: number, end: number, step: number = 1) => Array.from({ length: (end - start) / step }, (_, i) => (i * step) + start);

/** generate n points evenly spaced between start and end: [start,end]
 * ```
 * linspace(0,1,5) // [0,0.25,0.5,0.75,1]
 * ```
 * */
export const linspace = (start: number, end: number, numPoints: number): number[] => {
  const segment = (end - start) / (numPoints - 1)
  return range(0, numPoints).map(i => start + i * segment)
}
