export type ActionKind =
  | "new_account"
  | "new_transaction"
  | "delete_account"
  | "open_credit"
  | "";

export interface IAccount {
  card_number: string;
  balance: number;
  transactions: ITransactions[];
  exists: boolean;
  tokens: string[];
  username: string;
}

interface ITransactions {
  amount: number;
  datetime: string;
  recipient: IInterlocutor;
  sender: IInterlocutor;
}

interface IInterlocutor {
  card_number: string;
  is_existing: boolean;
  username: string;
}

export interface IStoreInfo {
  card: {
    content: string;
    is_loading: boolean;
  };
  balance: {
    content: string;
    is_loading: boolean;
  };
  emission: {
    content: string;
    is_loading: boolean;
  };
}
