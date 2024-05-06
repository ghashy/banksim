import { PayloadAction, createSlice } from "@reduxjs/toolkit";

interface InitialState {
  is_open: boolean;
}

const initial_state: InitialState = {
  is_open: true,
};

const account_socket_slice = createSlice({
  name: "account_socket",
  initialState: initial_state,
  reducers: {
    set_account_socket_open: (
      state: InitialState,
      action: PayloadAction<boolean>
    ) => {
      state.is_open = action.payload;
    },
  },
});

export const { set_account_socket_open } = account_socket_slice.actions;

export default account_socket_slice.reducer;
