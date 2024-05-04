import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { IAccount } from "../types";

interface InitialState {
  account_list: IAccount[];
  is_loading: boolean;
}

const initial_state: InitialState = {
  account_list: [],
  is_loading: true,
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
  },
});

export const { set_acccount_list, set_accounts_loading } =
  account_list_slice.actions;

export default account_list_slice.reducer;
