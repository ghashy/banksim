import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { IStoreInfo } from "../types";

const initial_state: IStoreInfo = {
  card: "",
  balance: "",
  emission: "",
};

const store_info_slice = createSlice({
  name: "store_info",
  initialState: initial_state,
  reducers: {
    set_store_card: (state: IStoreInfo, action: PayloadAction<string>) => {
      state.card = action.payload;
    },
    set_store_balance: (state: IStoreInfo, action: PayloadAction<string>) => {
      state.balance = action.payload;
    },
    set_store_emission: (state: IStoreInfo, action: PayloadAction<string>) => {
      state.emission = action.payload;
    },
  },
});

export const { set_store_card, set_store_balance, set_store_emission } =
  store_info_slice.actions;

export default store_info_slice.reducer;
