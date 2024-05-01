export type ActionKind =
  | "new_account"
  | "new_transaction"
  | "delete_account"
  | "open_credit"
  | "";

export interface IAccount {
  card_number: string;
  balance: number;
  transactions: number;
  exists: boolean;
  tokens: string[];
  username: string;
}
