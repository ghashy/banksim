export function format_price(price: number): string {
  return price.toLocaleString().replace(/,/g, "_");
}

export const wait = (ms: number) => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};

export function handle_retry(
  error_response_status: number,
  set_states: () => void
) {
  if (error_response_status) {
    set_states();
  } else {
    location.reload();
  }
}
