import { PayloadAction, createSlice } from "@reduxjs/toolkit";

interface InitialState {
  logs: string[];
}

const initial_state: InitialState = {
  logs: [],
};

const logs_slice = createSlice({
  name: "logs",
  initialState: initial_state,
  reducers: {
    set_logs: (state: InitialState, action: PayloadAction<string>) => {
      state.logs = [...state.logs, action.payload];
    },
  },
});

export const { set_logs } = logs_slice.actions;

export default logs_slice.reducer;
