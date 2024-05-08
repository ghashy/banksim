export function format_price(price: number): string {
  return price.toLocaleString().replace(/,/g, "_");
}

export const wait = (ms: number) => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};
