// https://2021.adventjs.dev/challenges/08

export default function maxProfit(prices) {
  let maxProfit = -1;
  
  for (let i = 0; i < prices.length - 1; ++i) {
    for (let j = i + 1; j < prices.length; ++j) {
      const diff = prices[j] - prices[i];
      if (diff > 0 && diff > maxProfit) {
        maxProfit = diff;
      }
    }
  }

  return maxProfit;
}
