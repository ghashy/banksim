import { configureStore } from "@reduxjs/toolkit";
import checked_items_reducer from "./checked_items_slice";
import account_list_reducer from "./account_list_slice";
import store_info_reducer from "./store_info_slice";
import logs_reducer from "./logs_slice";
import socket_open_reducer from "./socket_open_slice";

const store = configureStore({
  reducer: {
    checked_items: checked_items_reducer,
    account_list: account_list_reducer,
    store_info: store_info_reducer,
    logs: logs_reducer,
    socket_open: socket_open_reducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export default store;
