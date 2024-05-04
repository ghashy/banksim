import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { IAccount } from "../types";

interface InitialState {
  account_list: IAccount[];
  is_loading: boolean;
  error: string;
}

const initial_state: InitialState = {
  account_list: [],
  is_loading: true,
  error: "",
};

const account_list_slice = createSlice({
  name: "account_list",
  initialState: initial_state,
  reducers: {
    set_acccount_list: (
      state: InitialState,
      action: PayloadAction<IAccount[]>
    ) => {
      state.account_list = action.payload;
    },
    set_accounts_loading: (
      state: InitialState,
      action: PayloadAction<boolean>
    ) => {
      state.is_loading = action.payload;
    },
    set_accounts_error: (
      state: InitialState,
      action: PayloadAction<string>
    ) => {
      state.error = action.payload;
    },
  },
});

export const { set_acccount_list, set_accounts_loading, set_accounts_error } =
  account_list_slice.actions;

export default account_list_slice.reducer;
