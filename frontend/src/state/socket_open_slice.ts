import { PayloadAction, createSlice } from "@reduxjs/toolkit";

interface InitialState {
  accounts_open: boolean;
  logs_open: boolean;
}

const initial_state: InitialState = {
  accounts_open: true,
  logs_open: true,
};

const socket_open_slice = createSlice({
  name: "socket_open",
  initialState: initial_state,
  reducers: {
    set_account_socket_open: (
      state: InitialState,
      action: PayloadAction<boolean>
    ) => {
      state.accounts_open = action.payload;
    },
    set_logs_socket_open: (
      state: InitialState,
      action: PayloadAction<boolean>
    ) => {
      state.logs_open = action.payload;
    },
  },
});

export const { set_account_socket_open, set_logs_socket_open } =
  socket_open_slice.actions;

export default socket_open_slice.reducer;
