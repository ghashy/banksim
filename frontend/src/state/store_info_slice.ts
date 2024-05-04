import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { IStoreInfo } from "../types";

const initial_state: IStoreInfo = {
  card: {
    content: "--",
    is_loading: true,
  },
  balance: {
    content: "--",
    is_loading: true,
  },
  emission: {
    content: "--",
    is_loading: true,
  },
};

const store_info_slice = createSlice({
  name: "store_info",
  initialState: initial_state,
  reducers: {
    set_store_card: (state: IStoreInfo, action: PayloadAction<string>) => {
      state.card.content = action.payload;
    },
    set_store_card_loading: (
      state: IStoreInfo,
      action: PayloadAction<boolean>
    ) => {
      state.card.is_loading = action.payload;
    },
    set_store_balance: (state: IStoreInfo, action: PayloadAction<string>) => {
      state.balance.content = action.payload;
    },
    set_store_balance_loading: (
      state: IStoreInfo,
      action: PayloadAction<boolean>
    ) => {
      state.balance.is_loading = action.payload;
    },
    set_store_emission: (state: IStoreInfo, action: PayloadAction<string>) => {
      state.emission.content = action.payload;
    },
    set_store_emmision_loading: (
      state: IStoreInfo,
      action: PayloadAction<boolean>
    ) => {
      state.emission.is_loading = action.payload;
    },
  },
});

export const {
  set_store_card,
  set_store_balance,
  set_store_emission,
  set_store_card_loading,
  set_store_balance_loading,
  set_store_emmision_loading,
} = store_info_slice.actions;

export default store_info_slice.reducer;
