export function format_price(price: number): string {
  return price.toLocaleString().replace(/,/g, "_");
}
