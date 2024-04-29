import { PayloadAction, createSlice } from "@reduxjs/toolkit";

interface InitialState {
  items: string[];
}
const initial_state: InitialState = {
  items: [],
};

const checked_items_slice = createSlice({
  name: "checked_items",
  initialState: initial_state,
  reducers: {
    set_checked_items: (
      state: InitialState,
      action: PayloadAction<string[]>
    ) => {
      state.items = action.payload;
    },
    reset_checked_itmes: () => initial_state,
  },
});

export const { set_checked_items, reset_checked_itmes } =
  checked_items_slice.actions;

export default checked_items_slice.reducer;
