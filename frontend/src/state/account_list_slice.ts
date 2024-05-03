import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { IAccount } from "../types";

interface InitialState {
  account_list: IAccount[];
}

const initial_state: InitialState = {
  account_list: [],
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
  },
});

export const { set_acccount_list } = account_list_slice.actions;

export default account_list_slice.reducer;
